# itsuki

macro that defines a simple zero-based sequential enum ⭐️

## Usage

```rs
use itsuki::zero_indexed_enum;

zero_indexed_enum! {
    Quintuplets => [Ichika, Nino, Miku, Yotsuba, Itsuki]
}

let miku = Quintuplets::Miku;

assert_eq!(miku, Quintuplets::Miku);
assert_ne!(miku, Quintuplets::Nino);

assert_eq!(Quintuplets::len(), 5);

assert_eq!(Quintuplets::Nino.next(), Quintuplets::Miku);
assert_eq!(Quintuplets::Itsuki.next(), Quintuplets::Ichika);

assert_eq!(Quintuplets::Yotsuba.prev(), Quintuplets::Miku);
assert_eq!(Quintuplets::Ichika.prev(), Quintuplets::Itsuki);

assert_eq!(Quintuplets::Miku.val(), 2);
assert_eq!(Quintuplets::Yotsuba.val(), 3);

assert_eq!(Quintuplets::try_from(0), Ok(Quintuplets::Ichika));
assert_eq!(Quintuplets::try_from(4), Ok(Quintuplets::Itsuki));
assert_eq!(Quintuplets::try_from(5), Err(()));

assert_eq!(1.try_into(), Ok(Quintuplets::Nino));
assert_eq!(3.try_into(), Ok(Quintuplets::Yotsuba));
```

## License

MIT
