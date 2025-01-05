use core::f64;
use ordered_float::OrderedFloat;
use std::{cell::RefCell, collections::BinaryHeap};

#[derive(Default)]
pub struct CalculateGCD {
    input: RefCell<Vec<f64>>,
    queue: RefCell<BinaryHeap<OrderedFloat<f64>>>,
}

impl Clone for CalculateGCD {
    fn clone(&self) -> Self {
        Self::default()
    }
}

impl CalculateGCD {
    fn try_determine_gcd(hint: f64, values: &Vec<f64>, integrality_tolerance: f64) -> Option<f64> {
        let mut s = 0.0;
        for a in values {
            let z = a / hint;
            let rounded_z = z.round();
            if rounded_z == 0.0 {
                return None;
            }
            s += a / rounded_z - hint;
        }
        let x = hint + s / values.len() as f64;
        if values.iter().all(|&a| {
            let z = a / x;
            (z.round() - z).abs() <= integrality_tolerance
        }) {
            return Some(x);
        } else {
            return None;
        }
    }

    fn calculate<I>(&self, values: I) -> f64
    where
        I: Iterator<Item = f64>,
    {
        let integrality_tolerance = 1e-8;

        let mut input = self.input.borrow_mut();
        let mut queue = self.queue.borrow_mut();

        // 0 と±∞を除いて input に格納
        input.clear();
        input.extend(values.filter(|&x| x != 0.0 && x.is_finite()));

        if input.is_empty() || input.iter().any(|x| x.is_nan()) {
            // 空または NAN を含んでいれば gcd が定まらないので NAN を返す
            return f64::NAN;
        } else if input.len() == 1 {
            // 有効な値が 1 つだけであればそれを返す
            return input[0];
        }

        // gcd の最大値
        let eps = 1e-8 * input.iter().max_by(|&l, &r| l.partial_cmp(r).unwrap()).unwrap();

        // queue に値を格納
        queue.clear();
        {
            let min_value_index = (0..input.len()).min_by(|&l, &r| input[l].partial_cmp(&input[r]).unwrap()).unwrap();
            debug_assert!(input[min_value_index] > 0.0);
            for i in 0..input.len() {
                queue.push(if i == min_value_index { input[i] } else { input[i] % input[min_value_index] }.into());
            }
        }

        loop {
            // 最大要素を取り出す
            let max_value = queue.pop().unwrap().into_inner();
            debug_assert!(max_value > 0.0);
            // 2 番目に大きい要素を取得
            let second_value = queue.peek().unwrap().into_inner();
            // eprintln!("{} {}", max_value, second_value);
            // 最大要素が GCD になっているかを確認
            if second_value < 0.5 * max_value {
                if let Some(gcd) = Self::try_determine_gcd(max_value, &input, integrality_tolerance) {
                    return gcd;
                }
            }
            // 2 番目に大きい要素(次の反復での最大要素)が eps 未満であれば中断
            if second_value < eps {
                return f64::NAN;
            }
            // 最大要素を 2 番目の要素で割った余りをキューに追加
            queue.push((max_value % second_value).into());
        }
    }

    // fn calculate2<I>(&self, values: I) -> f64
    // where
    //     I: Iterator<Item=f64>,
    // {
    //     let integrality_tolerance = 1e-8;

    //     let mut input = self.input.borrow_mut();
    //     let mut work = self.work.borrow_mut();

    //     // 0 と±∞を除いて input に格納
    //     input.clear();
    //     input.extend(values.filter(|&x| x != 0.0 && x.is_finite()));

    //     if input.is_empty() || input.iter().any(|x| x.is_nan()){
    //         // 空または NAN を含んでいれば gcd が定まらないので NAN を返す
    //         return f64::NAN;
    //     } else if input.len() == 1 {
    //         // 有効な値が 1 つだけであればそれを返す
    //         return input[0];
    //     }

    //     // gcd の最大値
    //     let eps = 1e-10 * input.iter().max_by(|&l, &r| l.partial_cmp(r).unwrap()).unwrap();

    //     // work に値を格納
    //     work.clear();
    //     work.extend(input.iter());

    //     loop {
    //         // 最小要素を探索
    //         let Some(min_value_index) = ({
    //             let mut i = None;
    //             for k in 0..work.len() {
    //                 if work[k] > eps && i.is_none_or(|i| work[i] > work[k]) {
    //                     i = k.into();
    //                 }
    //             }
    //             i
    //         }) else {
    //             return f64::NAN;
    //         };
    //         debug_assert!(work[min_value_index] > 0.0);
    //         // 最小要素が GCD になっているかを確認
    //         if let Some(gcd) = Self::try_determine_gcd(work[min_value_index], &input, integrality_tolerance) {
    //             return gcd;
    //         }
    //         //
    //         let mut flag = false;
    //         for k in 0..work.len() {
    //             if k != min_value_index && work[k] > eps {
    //                 flag = true;
    //                 work[k] %= work[min_value_index];
    //             }
    //         }
    //         if !flag {
    //             return f64::NAN;
    //         }
    //     }
    // }
}

impl<I> FnOnce<(I,)> for CalculateGCD
where
    I: Iterator<Item = f64>,
{
    type Output = f64;
    extern "rust-call" fn call_once(self, (values,): (I,)) -> Self::Output {
        self.calculate(values)
    }
}

impl<I> FnMut<(I,)> for CalculateGCD
where
    I: Iterator<Item = f64>,
{
    extern "rust-call" fn call_mut(&mut self, (values,): (I,)) -> Self::Output {
        self.calculate(values)
    }
}

impl<I> Fn<(I,)> for CalculateGCD
where
    I: Iterator<Item = f64>,
{
    extern "rust-call" fn call(&self, (values,): (I,)) -> Self::Output {
        self.calculate(values)
    }
}

#[cfg(test)]
mod test {

    use core::f64;
    use rand::{Rng, SeedableRng, distributions::Uniform, rngs::SmallRng};

    use super::CalculateGCD;

    #[test]
    fn test() {
        let calculate_gcd = CalculateGCD::default();
        dbg!(calculate_gcd([4.0, 2.0].into_iter()));
        dbg!(calculate_gcd([2.0, 6.0].into_iter()));
        dbg!(calculate_gcd([1071.0, 1029.0, 63.0].into_iter()));
        dbg!(calculate_gcd([1071.0, 1029.0, 63.0].iter().map(|x| x * f64::consts::PI)) / f64::consts::PI);
    }

    // #[test]
    // fn test1() {
    //     let calculate_gcd = CalculateGCD::default();
    //     let gcd = calculate_gcd(
    //         [9115861, 3579842, 8857072, 5522688, 4194833, 8947736, 7134865, 9740747, 6159140, 6773809].iter().map(|&x| x as f64 * 7.4269920097560895)
    //     );
    //     dbg!(gcd);
    //     assert!(gcd > 0.0);
    // }

    #[test]
    fn test2() {
        let calculate_gcd = CalculateGCD::default();
        let mut rng = SmallRng::seed_from_u64(42);

        for _ in 0..10000 {
            let data = Vec::from_iter((0..1000).map(|_| rng.gen_range(1..100000)));
            // for &x in data.iter() {
            //     eprint!("{} ", x);
            // }
            // eprint!("\n");
            let a = rng.sample(Uniform::new(1.0, 10.0));
            let gcd = calculate_gcd(data.iter().map(|&x| a * x as f64));
            if !(gcd > 0.0) {
                eprintln!("{}", a);
                for &x in data.iter() {
                    eprint!("{}, ", x);
                }
                eprint!("\n");
            }
            assert!(gcd > 0.0);
            // eprintln!("{}", gcd);
            // if gcd.is_nan() {
            // } else {
            //     for &x in data.iter() {
            //         let y = a * x as f64 / gcd;
            //         eprint!("{} ", y);
            //         assert!((y - y.round()).abs() <= 1e-2);
            //     }
            //     eprint!("\n");
            // }
        }
    }
}
