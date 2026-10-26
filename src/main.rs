use leptos::{html, prelude::*};
use serde::{Deserialize, Serialize};

const STORAGE_KEY: &str = "floured-apron-content-v1";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Content {
    headline: String,
    intro: String,
    announcement: String,
    products: Vec<Product>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Product {
    id: String,
    name: String,
    category: String,
    description: String,
    price: u32,
    unit: String,
    image: String,
    tag: String,
    available: bool,
    variants: Vec<Variant>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Variant {
    label: String,
    price: u32,
}

#[derive(Clone, Debug)]
struct CartItem {
    product_id: String,
    name: String,
    variant: String,
    price: u32,
    quantity: u32,
}

fn defaults() -> Content {
    serde_json::from_str(include_str!("../content.json")).expect("valid bundled content")
}
fn load_content() -> Content {
    web_sys::window()
        .and_then(|w| w.local_storage().ok().flatten())
        .and_then(|s| s.get_item(STORAGE_KEY).ok().flatten())
        .and_then(|s| serde_json::from_str::<Content>(&s).ok())
        .filter(|c| !c.products.is_empty() && c.products.len() <= 100)
        .unwrap_or_else(defaults)
}
fn open_dialog(dialog: NodeRef<html::Dialog>) {
    if let Some(d) = dialog.get() {
        let _ = d.show_modal();
    }
}
fn close_dialog(dialog: NodeRef<html::Dialog>) {
    if let Some(d) = dialog.get() {
        d.close();
    }
}
fn money(value: u32) -> String {
    format!("${value}")
}

#[component]
fn Arrow() -> impl IntoView {
    view! { <svg aria-hidden="true" width="20" height="20" viewBox="0 0 24 24" fill="none"><path d="M4 12h15m-6-6 6 6-6 6" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/></svg> }
}
#[component]
fn Bag() -> impl IntoView {
    view! { <svg aria-hidden="true" width="20" height="22" viewBox="0 0 24 24" fill="none"><path d="M5 7h14l1 14H4L5 7Z M8 8V6a4 4 0 0 1 8 0v2" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/></svg> }
}
#[component]
fn Wheat() -> impl IntoView {
    view! { <svg aria-hidden="true" class="wheat" viewBox="0 0 44 64" fill="none"><path d="M22 62V18M22 20C9 19 9 8 10 5c9 2 12 8 12 15Zm0 12C8 31 5 20 6 17c11 1 16 8 16 15Zm0 13C7 44 3 33 4 29c12 1 18 8 18 16Zm0-25C35 19 35 8 34 5c-9 2-12 8-12 15Zm0 12c14-1 17-12 16-15-11 1-16 8-16 15Zm0 13c15-1 19-12 18-16-12 1-18 8-18 16Z" stroke="currentColor" stroke-width="1.3" stroke-linejoin="round"/></svg> }
}

#[component]
fn App() -> impl IntoView {
    let content = RwSignal::new(load_content());
    let cart = RwSignal::new(Vec::<CartItem>::new());
    let category = RwSignal::new("All bakes".to_string());
    let selected = RwSignal::new(None::<String>);
    let selected_variant = RwSignal::new(0_usize);
    let quantity = RwSignal::new(1_u32);
    let note = RwSignal::new(String::new());
    let cart_message = RwSignal::new(String::new());
    let studio_message = RwSignal::new(String::new());
    let product_dialog = NodeRef::<html::Dialog>::new();
    let cart_dialog = NodeRef::<html::Dialog>::new();
    let studio_dialog = NodeRef::<html::Dialog>::new();
    let menu_dialog = NodeRef::<html::Dialog>::new();
    let product = Memo::new(move |_| {
        let id = selected.get();
        content
            .get()
            .products
            .into_iter()
            .find(|p| Some(&p.id) == id.as_ref())
    });
    let cart_count = Memo::new(move |_| cart.get().iter().map(|i| i.quantity).sum::<u32>());
    let cart_total =
        Memo::new(move |_| cart.get().iter().map(|i| i.quantity * i.price).sum::<u32>());
    let choose_product = move |id: String| {
        selected.set(Some(id));
        selected_variant.set(0);
        quantity.set(1);
        open_dialog(product_dialog);
    };
    let order_text = move || {
        let mut result =
            String::from("Hi The Floured Apron! I’d like to ask about this order:\n\n");
        for item in cart.get() {
            let variant = if item.variant.is_empty() {
                String::new()
            } else {
                format!(" — {}", item.variant)
            };
            result.push_str(&format!(
                "{} × {}{}: {}\n",
                item.quantity,
                item.name,
                variant,
                money(item.quantity * item.price)
            ));
        }
        result.push_str(&format!("\nEstimated total: {}\n", money(cart_total.get())));
        if !note.get().trim().is_empty() {
            result.push_str(&format!("\n{}\n", note.get().trim()));
        }
        result.push_str(
            "\nPlease confirm pricing, availability, and pickup arrangements. Thank you!",
        );
        result
    };

    view! {
        <a class="skip-link" href="#main">"Skip to content"</a>
        <div class="preview-bar"><span><span class="preview-dot"></span>"A fresh look for The Floured Apron"<span class="preview-small">" · Design preview"</span></span><button on:click=move |_| { studio_message.set(String::new()); open_dialog(studio_dialog); }>"Try the content studio"<Arrow/></button></div>
        <div class="announcement">{move || content.get().announcement}</div>
        <header class="site-header">
            <a href="#" class="brand" aria-label="The Floured Apron home"><img src="assets/logo.jpg" width="58" height="58" alt=""/><span>"the floured apron"<small>"SOURDOUGH & SOMETHING SWEET"</small></span></a>
            <nav class="desktop-nav" aria-label="Main navigation"><a href="#menu">"Our menu"</a><a href="#story">"Our little bakery"</a><a href="#how-to-order">"How to order"</a></nav>
            <div class="header-actions"><button class="bag-button" aria-label=move || format!("Open your order, {} items", cart_count.get()) on:click=move |_| open_dialog(cart_dialog)><Bag/><span class="bag-label">"Your order"</span><span class="bag-count">{move || cart_count.get()}</span></button><button class="mobile-menu-button" aria-label="Open navigation" on:click=move |_| open_dialog(menu_dialog)><span></span><span></span></button></div>
        </header>
        <main id="main">
            <section class="hero" aria-labelledby="hero-heading">
                <div class="hero-copy"><p class="eyebrow"><span class="small-flower">"✳"</span>"YOUR NEIGHBORHOOD HOME BAKERY"</p><h1 id="hero-heading">{move || {let h = content.get().headline; let mut lines = h.splitn(2, '\n'); let first = lines.next().unwrap_or("").to_string(); let second = lines.next().unwrap_or("").to_string(); view! {<span>{first}</span><em>{second}</em>}}}</h1><p class="hero-description">{move || content.get().intro}</p><div class="hero-actions"><a class="button button-primary" href="#menu">"Find your favorite"<Arrow/></a><a class="text-link" href="#how-to-order">"How to order"</a></div><div class="hero-local"><span class="sun-mark">"☼"</span><span>"A little flour. A lot of love."<small>"Baked in Florence, Arizona"</small></span></div></div>
                <div class="hero-image"><img src="assets/sourdough.jpg" alt="A rustic sourdough loaf on a wooden table, styled as inspiration for the bakery" width="1200" height="800" fetchpriority="high"/><div class="photo-caption">"THE SIMPLE JOY OF SOMETHING HANDCRAFTED"</div><div class="hero-seal"><Wheat/><span>"Made with care"</span><small>"SHARED WITH LOVE"</small></div></div>
            </section>
            <div class="values-strip"><span>"Handcrafted loaves"</span><span aria-hidden="true">"✳"</span><span>"Sweet little moments"</span><span aria-hidden="true">"✳"</span><span>"From our home to yours"</span><span aria-hidden="true">"✳"</span><span>"Florence, Arizona"</span></div>
            <section class="favorites section-wrap" aria-labelledby="favorites-heading"><div class="section-heading"><div><p class="eyebrow">"A FEW GOOD PLACES TO START"</p><h2 id="favorites-heading">"Meet your new "<em>"favorites."</em></h2></div><a class="text-link arrow-link" href="#menu">"Explore the whole menu"<Arrow/></a></div><div class="product-grid">
                {move || content.get().products.into_iter().take(3).map(|p| { let id = p.id.clone(); let name = p.name.clone(); view! {<article class="product-card"><button class="product-image-button" on:click=move |_| choose_product(id.clone()) aria-label=format!("View {} options", p.name)><img src=format!("assets/{}", p.image) alt=format!("{} — illustrative stock photograph", p.name) loading="lazy" width="600" height="700"/><span class="product-tag">{p.tag}</span><span class="image-add" aria-hidden="true">"↗"</span></button><div class="product-title-line"><h3>{p.name}</h3><span class="product-price">{money(p.price)}</span></div><p class="product-unit">{p.unit}</p><p class="product-description">{p.description}</p><span class="sr-only">{name}</span></article>}}).collect_view()}
            </div></section>
            <section id="story" class="story-section"><div class="story-image"><img src="assets/baking-hands.jpg" alt="Hands working bread dough, an illustrative photograph of the craft of baking" loading="lazy" width="800" height="1000"/><span class="image-note">"Simple ingredients. Thoughtful hands."</span></div><div class="story-copy"><Wheat/><p class="eyebrow">"OUR LITTLE CORNER OF FLORENCE"</p><h2>"A home bakery."<br/>"A whole lot of "<em>"heart."</em></h2><p>"The Floured Apron is a home-based bakery in Florence, Arizona, with a love for good bread and the moments it brings people together."</p><p>"From a sourdough loaf for the dinner table to something sweet for the weekend, every order is a chance to share a little care. Have a flavor in mind? We’d love to hear it."</p><a class="text-link arrow-link" href="#contact">"Say hello"<Arrow/></a><span class="story-signature">"Made for your table."</span></div></section>
            <section id="menu" class="menu-section section-wrap" aria-labelledby="menu-heading"><div class="section-heading"><div><p class="eyebrow">"SOMETHING FOR EVERY KIND OF CRAVING"</p><h2 id="menu-heading">"The "<em>"good stuff."</em></h2></div><p class="section-aside">"Find a familiar favorite."<br/>"Try a little something new."</p></div><div class="menu-tools"><div class="category-tabs" role="group" aria-label="Filter menu by category">{["All bakes", "Sourdough", "Sweet loaves", "Sweet treats"].into_iter().map(|label| view! {<button class:active=move || category.get() == label aria-pressed=move || (category.get() == label).to_string() on:click=move |_| category.set(label.to_string())>{label}</button>}).collect_view()}</div><span class="menu-note">"Prices from the current bakery menu"</span></div><div class="menu-list" aria-live="polite">
                {move || content.get().products.into_iter().filter(|p| category.get() == "All bakes" || p.category == category.get()).map(|p| {let id = p.id.clone(); view! {<article class="menu-item"><div class="menu-item-info"><span class="menu-category">{p.category}</span><h3>{p.name.clone()}</h3><p>{p.description}</p></div><div class="menu-item-action"><span class="menu-item-price">{money(p.price)}<small>{p.unit}</small></span><button class="round-add" disabled=!p.available aria-label=format!("Choose {}", p.name) on:click=move |_| choose_product(id.clone())>{if p.available { "+" } else { "—" }}</button>{(!p.available).then(|| view!{<span class="sold-out">"Unavailable"</span>})}</div></article>}}).collect_view()}
            </div><p class="menu-footnote">"A particular flavor in mind? Custom requests are always welcome. Availability, ingredients, and pickup details are confirmed with the bakery."</p></section>
            <section id="how-to-order" class="order-section"><div class="section-wrap"><div class="section-heading"><div><p class="eyebrow">"GOOD THINGS ARE WORTH PLANNING FOR"</p><h2>"From our oven to "<em>"your table."</em></h2></div></div><div class="steps-grid"><article><span class="step-number">"01"</span><h3>"Pick your favorites"</h3><p>"Explore the menu, choose your flavors, and put together something delicious."</p></article><article><span class="step-number">"02"</span><h3>"Let’s make a plan"</h3><p>"Send your request to the bakery. We’ll confirm availability, the total, and pickup arrangements."</p></article><article><span class="step-number">"03"</span><h3>"Bring a little joy home"</h3><p>"Collect your order at the agreed time. Slice, share, and enjoy every last crumb."</p></article></div><div class="order-note"><span aria-hidden="true">"✳"</span><p>"This is a design preview. Try building an order—nothing will be sent, charged, or reserved."</p><button class="text-link arrow-link" on:click=move |_| open_dialog(cart_dialog)>"Try an order"<Arrow/></button></div></div></section>
            <section class="faq-section section-wrap"><div><p class="eyebrow">"A FEW HELPFUL DETAILS"</p><h2>"Before the "<br/>"first "<em>"bite."</em></h2></div><div class="faq-list"><details><summary>"Where is the bakery?"<span>"+"</span></summary><p>"The Floured Apron is a home-based bakery in Florence, Arizona. Contact the bakery to arrange pickup; it is not listed as a walk-in storefront."</p></details><details><summary>"How far ahead should I order?"<span>"+"</span></summary><p>"Please ask the bakery about your preferred date before making plans. Lead times and availability are confirmed personally with each request."</p></details><details><summary>"Can I request a flavor?"<span>"+"</span></summary><p>"Yes! The bakery welcomes flavor requests. Share your idea when you get in touch, and ask what’s possible for your order."</p></details><details><summary>"What about ingredients and allergies?"<span>"+"</span></summary><p>"Ask the bakery directly about ingredients and preparation before ordering. This preview does not make any allergen-free claims."</p></details></div></section>
            <section id="contact" class="contact-section"><span class="contact-flower" aria-hidden="true">"✳"</span><p class="eyebrow">"THERE’S ALWAYS ROOM AT THE TABLE"</p><h2>"Let’s bake something "<em>"good."</em></h2><p>"A favorite flavor, a special occasion, or just a question."<br/>"We’d love to hear from you."</p><a class="button button-light" href="https://theflouredapronaz.com/" target="_blank" rel="noopener noreferrer">"Visit the bakery’s current website"<Arrow/></a><span class="contact-location">"FLORENCE, ARIZONA"</span></section>
        </main>
        <footer class="site-footer"><div class="footer-top"><a href="#" class="footer-brand">"the floured apron"<small>"A little flour. A lot of love."</small></a><nav aria-label="Footer navigation"><a href="#menu">"Menu"</a><a href="#story">"Our story"</a><a href="https://www.instagram.com/theflouredapronaz" target="_blank" rel="noopener noreferrer">"Instagram ↗"</a><a href="https://www.facebook.com/profile.php?id=61552403211480" target="_blank" rel="noopener noreferrer">"Facebook ↗"</a></nav></div><div class="footer-bottom"><p>"Website concept for The Floured Apron · Florence, AZ"<br/>"Stock photography illustrates the design; it does not depict the bakery’s products."</p><button on:click=move |_| open_dialog(studio_dialog)>"Explore the content studio ↗"</button></div></footer>
        <button class="mobile-order-bar" on:click=move |_| open_dialog(cart_dialog)><span><Bag/>"Your order"<span class="bag-count">{move || cart_count.get()}</span></span><span>{move || if cart_count.get() > 0 { money(cart_total.get()) } else { "Take a look →".to_string() }}</span></button>

        <dialog aria-label="Navigation" node_ref=menu_dialog class="nav-dialog"><div class="dialog-top"><span class="eyebrow">"COME ON IN"</span><button class="close-button" aria-label="Close navigation" on:click=move |_| close_dialog(menu_dialog)>"×"</button></div><nav aria-label="Mobile navigation">{[("Our menu", "#menu"), ("Our little bakery", "#story"), ("How to order", "#how-to-order"), ("Say hello", "#contact")].into_iter().map(|(label,href)| view!{<a href=href on:click=move |_| close_dialog(menu_dialog)>{label}<Arrow/></a>}).collect_view()}</nav></dialog>

        <dialog aria-label="Customize your bake" node_ref=product_dialog class="product-dialog"><div class="dialog-top"><span class="eyebrow">"MAKE IT YOURS"</span><button class="close-button" aria-label="Close product options" on:click=move |_| close_dialog(product_dialog)>"×"</button></div>{move || product.get().map(|p| {let has_variants = !p.variants.is_empty(); view! {<div class="product-dialog-body"><p class="eyebrow">{p.category}</p><h2>{p.name}</h2><p>{p.description}</p><Show when=move || has_variants><label class="field-label" for="variant">"Choose your option"</label><select id="variant" prop:value=move || selected_variant.get().to_string() on:change=move |ev| selected_variant.set(event_target_value(&ev).parse().unwrap_or(0))>{p.variants.clone().into_iter().enumerate().map(|(i,v)|view!{<option value=i.to_string()>{format!("{} · {}",v.label,money(v.price))}</option>}).collect_view()}</select></Show><div class="quantity-row"><label for="product-quantity">"Quantity"</label><div class="stepper"><button aria-label="Decrease quantity" disabled={move || quantity.get() <= 1} on:click=move |_| quantity.update(|q| *q=q.saturating_sub(1).max(1))>"−"</button><input id="product-quantity" type="number" min="1" max="24" prop:value=move || quantity.get() on:change=move |ev| quantity.set(event_target_value(&ev).parse::<u32>().unwrap_or(1).clamp(1,24))/><button aria-label="Increase quantity" disabled={move || quantity.get() >= 24} on:click=move |_| quantity.update(|q| *q=(*q+1).min(24))>"+"</button></div></div><button class="button button-primary wide" disabled=!p.available on:click=move |_| {if let Some(p)=product.get(){let v=p.variants.get(selected_variant.get());let price=v.map_or(p.price,|v|v.price);let variant=v.map_or_else(String::new,|v|v.label.clone());cart.update(|items|{if let Some(item)=items.iter_mut().find(|i|i.product_id==p.id&&i.variant==variant){item.quantity=(item.quantity+quantity.get()).min(99);}else{items.push(CartItem{product_id:p.id,name:p.name,variant,price,quantity:quantity.get()});}});cart_message.set(String::new());close_dialog(product_dialog);open_dialog(cart_dialog);}}>"Add to your order"<span>{move || product.get().map(|p| money(p.variants.get(selected_variant.get()).map_or(p.price,|v|v.price)*quantity.get())).unwrap_or_default()}</span></button><p class="fine-print">"Preview only. Availability and final pricing would be confirmed by the bakery."</p></div>}})}</dialog>

        <dialog aria-label="Your order" node_ref=cart_dialog class="cart-dialog"><div class="dialog-top"><span class="eyebrow">"A LITTLE SOMETHING DELICIOUS"</span><button class="close-button" aria-label="Close your order" on:click=move |_| close_dialog(cart_dialog)>"×"</button></div><h2>"Your "<em>"order."</em></h2><p class="dialog-intro">"A few good things, ready to share."</p><Show when={move || cart_count.get() > 0} fallback=move || view!{<div class="empty-cart"><Wheat/><h3>"Something good starts here."</h3><p>"Add a loaf, a sweet treat, or a little of both."</p><a class="button button-primary" href="#menu" on:click=move |_| close_dialog(cart_dialog)>"Explore the menu"<Arrow/></a></div>}><div class="cart-items">{move || cart.get().into_iter().enumerate().map(|(index,item)|view!{<article class="cart-item"><div><h3>{item.name}</h3><p>{item.variant}</p><div class="cart-item-controls"><button aria-label=format!("Decrease item {} quantity",index+1) on:click=move |_| {cart.update(|items|{if let Some(i)=items.get_mut(index){i.quantity=i.quantity.saturating_sub(1);}items.retain(|i|i.quantity>0);});cart_message.set(String::new());}>"−"</button><span>{item.quantity}</span><button aria-label=format!("Increase item {} quantity",index+1) disabled={item.quantity >= 99} on:click=move |_| {cart.update(|items|{if let Some(i)=items.get_mut(index){i.quantity=(i.quantity+1).min(99);}});cart_message.set(String::new());}>"+"</button><button class="remove-button" on:click=move |_| {cart.update(|items|{if index<items.len(){items.remove(index);}});cart_message.set(String::new());}>"Remove"</button></div></div><strong>{money(item.price*item.quantity)}</strong></article>}).collect_view()}</div><div class="cart-total"><span>"Estimated total"</span><strong>{move || money(cart_total.get())}</strong></div><label class="field-label" for="order-note">"Anything you’d like to ask?"</label><textarea id="order-note" rows="3" maxlength="1000" placeholder="A preferred pickup day, a flavor request…" prop:value=move || note.get() on:input=move |ev| {note.set(event_target_value(&ev));cart_message.set(String::new());}></textarea><button class="button button-primary wide" on:click=move |_| {let text=order_text();leptos::task::spawn_local(async move {if let Some(w)=web_sys::window(){let promise=w.navigator().clipboard().write_text(&text);match wasm_bindgen_futures::JsFuture::from(promise).await{Ok(_)=>cart_message.set("Copied! This is a draft only; nothing has been sent.".to_string()),Err(_)=>cart_message.set("Your browser couldn’t copy automatically. Open the draft below to select and copy it.".to_string())}}});}>"Copy order request"<Arrow/></button><p class="status-message" role="status">{move || cart_message.get()}</p><details class="order-draft"><summary>"View your order draft"</summary><pre>{order_text}</pre></details><a class="continue-shopping" href="#menu" on:click=move |_| close_dialog(cart_dialog)>"Keep exploring the menu"</a></Show><p class="demo-notice">"Design preview · No order is sent, charged, or reserved. Prices and availability must be confirmed with the bakery."</p></dialog>

        <dialog aria-label="Content studio" node_ref=studio_dialog class="studio-dialog"><div class="dialog-top"><span class="eyebrow">"THE FLOURED APRON / CONTENT STUDIO"</span><button class="close-button" aria-label="Close content studio" on:click=move |_| close_dialog(studio_dialog)>"×"</button></div><h2>"Your bakery. "<em>"Your words."</em></h2><p class="dialog-intro">"Try updating the homepage or menu. Changes appear immediately in this preview and can be saved on this browser."</p><div class="studio-explainer">"This is a CMS interaction demo. It doesn’t publish changes for other visitors or connect to a production server."</div><div class="studio-layout"><section><h3>"The first impression"</h3><label class="field-label" for="edit-headline">"Headline · two lines"</label><textarea id="edit-headline" rows="2" maxlength="100" prop:value=move || content.get().headline on:input=move |ev| {content.update(|c|c.headline=event_target_value(&ev));studio_message.set(String::new());}></textarea><label class="field-label" for="edit-intro">"A little introduction"</label><textarea id="edit-intro" rows="4" maxlength="320" prop:value=move || content.get().intro on:input=move |ev| {content.update(|c|c.intro=event_target_value(&ev));studio_message.set(String::new());}></textarea><label class="field-label" for="edit-announcement">"Announcement bar"</label><input id="edit-announcement" maxlength="140" prop:value=move || content.get().announcement on:input=move |ev| {content.update(|c|c.announcement=event_target_value(&ev));studio_message.set(String::new());}/><p class="fine-print">"Change an announcement for a seasonal menu, a holiday, or a bakery update."</p></section><section><h3>"What’s on the menu"</h3><p class="fine-print">"Set a base price or mark an item unavailable. Variant prices keep their original difference from the base price."</p><div class="studio-products">{defaults().products.into_iter().map(|p|{let id1=p.id.clone();let id2=p.id.clone();let id3=p.id.clone();let id4=p.id.clone();view!{<div class="studio-product"><span>{p.name.clone()}</span><label><span class="sr-only">{format!("{} price in dollars",p.name)}</span><span aria-hidden="true">"$"</span><input type="number" min="1" max="500" aria-label=format!("{} price in dollars",p.name) prop:value=move || content.get().products.iter().find(|p|p.id==id1).map_or(0,|p|p.price) on:change=move |ev| {let price=event_target_value(&ev).parse::<u32>().unwrap_or(1).clamp(1,500);content.update(|c|{if let Some(p)=c.products.iter_mut().find(|p|p.id==id2){let delta=price as i64-p.price as i64;for v in &mut p.variants{v.price=(v.price as i64+delta).clamp(1,1000) as u32;}p.price=price;}});cart.set(vec![]);cart_message.set(String::new());studio_message.set("Prices updated. The sample order was cleared so it uses the new prices.".to_string());}/></label><label class="availability-control"><input type="checkbox" aria-label=format!("{} available",p.name) prop:checked=move || content.get().products.iter().find(|p|p.id==id3).is_some_and(|p|p.available) on:change=move |ev| {content.update(|c|{if let Some(p)=c.products.iter_mut().find(|p|p.id==id4){p.available=event_target_checked(&ev);}});cart.set(vec![]);studio_message.set("Availability updated. The sample order was cleared.".to_string());}/><span>"Available"</span></label></div>}}).collect_view()}</div></section></div><div class="studio-actions"><button class="button button-primary" on:click=move |_| {let result=web_sys::window().and_then(|w|w.local_storage().ok().flatten()).map(|s|s.set_item(STORAGE_KEY,&serde_json::to_string(&content.get()).unwrap_or_default()));studio_message.set(if matches!(result,Some(Ok(()))){"Saved in this browser. Close the studio to see your changes."}else{"Your browser blocked storage. Changes remain visible until you reload."}.to_string());}>"Save on this browser"<Arrow/></button><button class="button button-outline" on:click=move |_| {content.set(defaults());cart.set(vec![]);if let Some(s)=web_sys::window().and_then(|w|w.local_storage().ok().flatten()){let _=s.remove_item(STORAGE_KEY);}studio_message.set("Original preview restored.".to_string());}>"Restore original"</button></div><p class="status-message" role="status">{move || studio_message.get()}</p></dialog>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn catalog_has_unique_ids_and_consistent_variant_prices() {
        let content = defaults();
        let mut ids = std::collections::HashSet::new();
        for p in content.products {
            assert!(ids.insert(p.id), "duplicate product id");
            assert!(p.price > 0);
            if let Some(first) = p.variants.first() {
                assert_eq!(first.price, p.price);
            }
        }
    }
    #[test]
    fn verified_menu_prices_are_preserved() {
        let c = defaults();
        let find = |id: &str| c.products.iter().find(|p| p.id == id).unwrap();
        assert_eq!(find("sourdough").price, 12);
        assert_eq!(find("flavored").price, 15);
        assert_eq!(find("rolls").variants[1].price, 30);
        assert_eq!(find("muffins").unit, "half dozen");
    }
}
