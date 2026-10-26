async (page) => {
  const checks = [];
  const failures = [];
  const assert = (condition, message) => {
    if (!condition) throw new Error(message);
    checks.push(message);
  };
  const onError = error => failures.push(String(error));
  page.on('pageerror', onError);
  try {
    await page.setViewportSize({ width: 1440, height: 1000 });
    await page.reload();
    await page.getByRole('heading', { name: 'Good bread. Great company.' }).waitFor();
    assert(await page.locator('.menu-item').count() === 9, 'All nine products render');
    assert(!await page.locator('body').innerText().then(t => t.includes('<br/>')), 'No raw line-break markup');
    await page.getByRole('button', { name: 'Sweet loaves', exact: true }).click();
    assert(await page.locator('.menu-item').count() === 4, 'Category filtering shows four sweet loaves');
    await page.getByRole('button', { name: 'All bakes', exact: true }).click();
    await page.getByRole('button', { name: 'Choose Cinnamon Rolls', exact: true }).click();
    await page.getByLabel('Choose your option').selectOption('1');
    await page.getByRole('button', { name: 'Increase quantity', exact: true }).click();
    assert((await page.getByRole('button', { name: /Add to your order/ }).innerText()).includes('$60'), 'Two full cinnamon-roll orders total $60');
    await page.getByRole('button', { name: /Add to your order/ }).click();
    assert(await page.locator('.cart-total strong').innerText() === '$60', 'Cart retains selected variant and quantity');
    await page.getByRole('button', { name: 'Close your order', exact: true }).click();
    await page.getByRole('button', { name: 'Choose Classic Sourdough', exact: true }).click();
    await page.getByRole('button', { name: /Add to your order/ }).click();
    assert(await page.locator('.cart-total strong').innerText() === '$72', 'Mixed order totals correctly');
    await page.getByRole('button', { name: 'Decrease item 1 quantity', exact: true }).click();
    assert(await page.locator('.cart-total strong').innerText() === '$42', 'Decreasing quantity updates totals');
    await page.locator('.cart-item').first().getByRole('button', { name: 'Remove', exact: true }).click();
    assert(await page.locator('.cart-total strong').innerText() === '$12', 'Removing a line updates totals');
    await page.getByLabel('Anything you’d like to ask?').fill('Would Saturday pickup be possible?');
    await page.getByText('View your order draft', { exact: true }).click();
    const draft = await page.locator('.order-draft pre').innerText();
    assert(draft.includes('Saturday') && draft.includes('$12') && draft.includes('Classic Sourdough'), 'Order draft includes items, total, and customer note');
    await page.getByRole('button', { name: 'Copy order request', exact: true }).click();
    await page.locator('.cart-dialog .status-message').waitFor({ state: 'visible' });
    assert((await page.locator('.cart-dialog .status-message').innerText()).includes('Copied!'), 'Order draft can be copied without sending');
    await page.keyboard.press('Escape');
    assert(await page.locator('dialog[open]').count() === 0, 'Escape closes the native order dialog');
    await page.getByRole('button', { name: 'Try the content studio', exact: true }).click();
    await page.getByLabel('Headline · two lines').fill('Fresh from the oven.\nMade for you.');
    const price = page.getByRole('spinbutton', { name: 'Classic Sourdough price in dollars', exact: true });
    await price.fill('14');
    await price.press('Tab');
    await page.getByRole('checkbox', { name: 'Cinnamon Rolls available', exact: true }).uncheck();
    await page.getByRole('button', { name: 'Save on this browser', exact: true }).click();
    assert((await page.locator('.studio-dialog .status-message').innerText()).includes('Saved in this browser'), 'Content editor saves locally');
    await page.getByRole('button', { name: 'Close content studio', exact: true }).click();
    assert((await page.locator('h1').innerText()).includes('Fresh from the oven.'), 'Homepage updates from content editor');
    assert(await page.getByRole('button', { name: 'Choose Cinnamon Rolls', exact: true }).isDisabled(), 'Unavailable items cannot be added from menu');
    await page.reload();
    await page.getByRole('heading', { name: 'Fresh from the oven. Made for you.' }).waitFor();
    assert((await page.locator('.menu-item').filter({ hasText: 'Classic Sourdough' }).innerText()).includes('$14'), 'Content edits persist through reload');
    await page.getByRole('button', { name: 'Try the content studio', exact: true }).click();
    await page.getByRole('button', { name: 'Restore original', exact: true }).click();
    await page.getByRole('button', { name: 'Close content studio', exact: true }).click();
    assert((await page.locator('h1').innerText()).includes('Good bread.'), 'Restore original resets the preview');
    for (const width of [1440, 768, 390, 320]) {
      await page.setViewportSize({ width, height: 844 });
      assert(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth), `No horizontal overflow at ${width}px`);
    }
    await page.setViewportSize({ width: 390, height: 844 });
    await page.getByRole('button', { name: 'Open navigation', exact: true }).click();
    await page.getByRole('navigation', { name: 'Mobile navigation' }).getByRole('link', { name: 'Our menu', exact: true }).click();
    assert(await page.locator('dialog[open]').count() === 0, 'Mobile navigation closes after selecting a section');
    await page.locator('.mobile-order-bar').click();
    assert(await page.getByRole('dialog', { name: 'Your order', exact: true }).isVisible(), 'Mobile order bar opens the order drawer');
    await page.keyboard.press('Escape');
    await page.locator('.faq-list summary').first().click();
    assert(await page.locator('.faq-list details').first().getAttribute('open') !== null, 'FAQ expands');
    await page.locator('.faq-list summary').first().click();
    await page.evaluate(async () => {
      document.querySelectorAll('img').forEach(image => image.loading = 'eager');
      await Promise.all([...document.images].map(image => image.decode().catch(() => {})));
    });
    assert(await page.evaluate(() => [...document.images].every(image => image.complete && image.naturalWidth > 0)), 'All photographs load');
    assert(failures.length === 0, 'No JavaScript or WebAssembly runtime errors');
    await page.evaluate(() => window.scrollTo({ top: 0, behavior: 'instant' }));
    return { passed: checks.length, checks, errors: failures };
  } finally {
    page.off('pageerror', onError);
  }
}
