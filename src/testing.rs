#[cfg(test)]
mod exact_match {
    use crate::{
        expr::Expr::{self, *},
        num::Num::*,
    };

    #[test]
    fn sum() {
        assert!(Expr::exact_match(
            &Sum(vec![Const(I), Const(E)]), // i + e
            &Sum(vec![Const(E), Const(I)]), // <match>
        ));
        assert!(/* NOT */ !Expr::exact_match(
            &Sum(vec![Const(Zero), Const(E)]), // 0 + e
            &Sum(vec![Const(E)]),              // e
        ));
        assert!(Expr::exact_match(
            &Sum(vec![Const(I), Const(E), Sum(vec![Const(One), Const(E)])]), // i + e + (1 + e)
            &Sum(vec![Const(E), Sum(vec![Const(E), Const(One)]), Const(I)]), // <match>
        ));
        assert!(/* NOT */ !Expr::exact_match(
            &Sum(vec![Const(I), Const(E), Sum(vec![Const(One), Const(E)])]), // i + e + (1 + e)
            &Sum(vec![Const(E), Const(E), Const(One), Const(I)]),            // i + e + 1 + e
        ));
    }

    #[test]
    fn product() {
        assert!(Expr::exact_match(
            &Product(vec![Const(I), Const(E)]), // i * e
            &Product(vec![Const(E), Const(I)]), // <match>
        ));
        assert!(/* NOT */ !Expr::exact_match(
            &Product(vec![Const(One), Const(E)]), // 1 * e
            &Product(vec![Const(E)]),             // e
        ));
        assert!(/* NOT */ !Expr::exact_match(
            &Product(vec![Const(Zero), Const(E)]), // 0 * e
            &Product(vec![Const(Zero)]),           // 0
        ));
        assert!(Expr::exact_match(
            &Product(vec![
                Const(I),
                Const(E),
                Product(vec![Const(One), Const(E)]),
            ]), // i * e * (1 * e)
            &Product(vec![
                Const(E),
                Product(vec![Const(E), Const(One)]),
                Const(I),
            ]), // <match>
        ));
        assert!(/* NOT */ !Expr::exact_match(
            &Product(vec![Const(I), Const(E), Sum(vec![Const(One), Const(E)])]), // i * e * (1 * e)
            &Product(vec![Const(E), Const(E), Const(One), Const(I)]),            // i * e * 1 * e
        ));
    }
}

#[cfg(test)]
mod simplify {
    use crate::{
        expr::Expr::{self, *},
        num::Num::*,
    };

    #[test]
    fn sum() {
        assert!(Expr::exact_match(
            &Sum(vec![
                Const(I),
                Const(Zero),
                Const(E),
                Sum(vec![Const(One), Const(Zero)]),
                Const(Zero),
                Const(Zero),
            ])
            // (i + 0 + e + (1 + 0) + 0 + 0)
            .simplify_trivial(),
            // i + e + 1
            &Sum(vec![Const(I), Const(E), Const(One)]),
        ));
    }

    #[test]
    fn product() {
        assert!(Expr::exact_match(
            &Product(vec![
                Const(I),
                Const(One),
                Const(E),
                Product(vec![Const(One), Const(I)]),
                Const(One),
                Const(One),
            ])
            // (i * 1 * e * (1 * i) * 1 * 1)
            .simplify_trivial(),
            // i * i * e
            &Product(vec![Const(I), Const(I), Const(E)])
        ));

        assert!(Expr::exact_match(
            &Product(vec![
                Const(I),
                Const(Zero),
                Const(E),
                Product(vec![Const(One), Const(I)]),
                Const(One),
                Const(One),
            ])
            // (i * 0 * e * (1 * i) * 1 * 1)
            .simplify_trivial(),
            // 0
            &Const(Zero)
        ));
    }
}
