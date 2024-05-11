# itsuki

[![Crate Status](https://img.shields.io/crates/v/itsuki.svg)](https://crates.io/crates/itsuki)

macro that defines a simple zero-based sequential enum ⭐️

## Usage

### Function-like

```rs
use itsuki::define_zero_indexed_enum;

define_zero_indexed_enum! {
    Quintuplets => [Ichika, Nino, Miku, Yotsuba, Itsuki]
}

let miku = Quintuplets::Miku;

assert_eq!(miku, Quintuplets::Miku);
assert_ne!(miku, Quintuplets::Nino);

assert_eq!(Quintuplets::len(), 5);

use Quintuplets::*;

assert_eq!(Quintuplets::vars_vec(), vec![Ichika, Nino, Miku, Yotsuba, Itsuki]);
assert_eq!(Quintuplets::vars_array(), [Ichika, Nino, Miku, Yotsuba, Itsuki]);

assert_eq!(Nino.next(), Miku);
assert_eq!(Itsuki.next(), Ichika);

assert_eq!(Yotsuba.prev(), Miku);
assert_eq!(Ichika.prev(), Itsuki);

assert_eq!(Ichika.next_in(|q| [Miku, Yotsuba].contains(&q)), Miku);
assert_eq!(Miku.next_in(|q| [Miku, Yotsuba].contains(&q)), Yotsuba);

assert_eq!(Nino.prev_in(|q| [Miku, Yotsuba].contains(&q)), Yotsuba);
assert_eq!(Yotsuba.prev_in(|q| [Miku, Yotsuba].contains(&q)), Miku);

assert_eq!(Miku.val(), 2);
assert_eq!(Yotsuba.val(), 3);

assert_eq!(Quintuplets::try_from(0), Ok(Ichika));
assert_eq!(Quintuplets::try_from(4), Ok(Itsuki));
assert_eq!(Quintuplets::try_from(5), Err(()));

assert_eq!(1.try_into(), Ok(Nino));
assert_eq!(3.try_into(), Ok(Yotsuba));
```

### Derive

```rs
use itsuki::ZeroIndexedEnum;

#[derive(ZeroIndexedEnum, Debug, Clone, Copy, PartialEq, Eq)]
enum Quintuplets {
    Ichika,
    Nino,
    Miku,
    Yotsuba,
    Itsuki,
}
```

## License

MIT
