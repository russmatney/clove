# CHANGELOG


## Untagged


## 


### 3 Jan 2025

- ([`30edac3`](https://github.com/russmatney/clove/commit/30edac3)) chore: more tweaks - Russell Matney

  > not sure about this rust version?

- ([`2537a52`](https://github.com/russmatney/clove/commit/2537a52)) deps: update tauri to 2.0 - Russell Matney

  > There might be a simpler way to create a webview now, but this seems to
  > work the same, at least.
  > 
  > There's still smearing/issues with transparency and updates, not sure
  > what that's about yet.


### 17 Oct 2023

- ([`6498013`](https://github.com/russmatney/clove/commit/6498013)) chore: update cargo deps, add note re: tauri nvidia error - Russell Matney

### 15 Aug 2023

- ([`a369434`](https://github.com/russmatney/clove/commit/a369434)) feat: support custom icons! - Russell Matney

  > You can now set a custom icon for your clove app, via `-i
  > path/to/icon.png`. Much thanks to @camsbury for the rust tips!


### 26 Jul 2023

- ([`5996c9a`](https://github.com/russmatney/clove/commit/5996c9a)) docs: examples using newest args - Russell Matney
- ([`d775741`](https://github.com/russmatney/clove/commit/d775741)) wip: some scratch code toward setting an icon - Russell Matney

  > It seems the tauri windowBuilder doesn't impl the builder pattern
  > properly b/c it consumes rather than borrows the struct, which seems to
  > prevent calling wb.icon() in some cases but not others. what a pita!

- ([`d086e9e`](https://github.com/russmatney/clove/commit/d086e9e)) feat: focused, transparent, decorations cli args - Russell Matney

  > Adds some boolean flags to expose options for setting auto-focus,
  > transparency, and window decorations for the given clove call.


### 25 Jul 2023

- ([`1081c40`](https://github.com/russmatney/clove/commit/1081c40)) misc: cargo updates, disable focus by default - Russell Matney

### 14 Jul 2023

- ([`4699515`](https://github.com/russmatney/clove/commit/4699515)) wip: toying with title bar style - Russell Matney
- ([`8a4198a`](https://github.com/russmatney/clove/commit/8a4198a)) feat: update cargo deps, remove hide-decorations - Russell Matney

  > hide-decorations on osx prevents apps from being closable, which is
  > total nonsense. Why would those things be coupled? Does it depend on the
  > menubar option simulating a click?


### 6 Jul 2023

- ([`7ca172c`](https://github.com/russmatney/clove/commit/7ca172c)) feat: update to tauri 1.4, cargo update - Russell Matney

### 18 Oct 2022

- ([`4bcbeb5`](https://github.com/russmatney/clove/commit/4bcbeb5)) fix: support transparency on osx - Russell Matney

  > Have to enable the macOSPrivateApi to make this work

- ([`34367a1`](https://github.com/russmatney/clove/commit/34367a1)) chore: blog links, drop some unused web stuff - Russell Matney

  > Really, none of `./src` is used... but it's fine as is for now,
  > if ever `clove` wants to take on related features.

- ([`39e1452`](https://github.com/russmatney/clove/commit/39e1452)) feat: window settings (title, transparency, etc) - Russell Matney

  > Plus docs for building, installing, usage.

- ([`0cc961d`](https://github.com/russmatney/clove/commit/0cc961d)) feat: creating tauri clients with arbitrary urls - Russell Matney

  > Note, title is not being set yet, and a handful of window attrs need to
  > be specified.

- ([`2ece655`](https://github.com/russmatney/clove/commit/2ece655)) fix: required change for cargo tauri build - Russell Matney

  > Prod build completing, so that's nice.
