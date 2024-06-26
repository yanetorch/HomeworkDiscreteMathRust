// Пользователь вводит вектор функции. Система строит СДНФ.


// алгоритм построения СДНФ :
// пропускаем строку, где функция принимает значение 0
// возвращаем ( x1 & x2 & x3 ), при которых функция принимает значение 1, и делаем отрицание переменных, у которых значение 0

use super::util;

// спорт прога стайл
fn get_expression(
    func: &util::BooleanFunction,
    func_value_skip: bool, // значение функции, которое скипаем
) -> String {
    let mut expr = String::new();
    let symbol = vec!['&', 'v'];
    let j = func_value_skip as usize;

    for (current_vals, f_val) in func {
        if f_val == func_value_skip {
            continue;
        }
        let mut line = String::new();

        if !expr.is_empty() {
            line.push(symbol[j ^ 1]);
            line.push('\n');
        }
        for (i, &value) in current_vals.iter().enumerate() {
            if i == 0 {
                line.push('(');
            }
            if value == !func_value_skip {
                line.push_str(&format!("x{}", i + 1));
            } else {
                line.push_str(&format!("-x{}", i + 1));
            }
            if i == current_vals.len() - 1 {
                line.push(')');
            } else {
                line.push(symbol[j]);
            }
        }
        expr.push_str(&line);
    }

    expr
}

pub fn get_fdnf(
    func: &util::BooleanFunction
) -> String {
    get_expression(func, false)
}

pub fn get_fknf(
    func: &util::BooleanFunction
) -> String {
    get_expression(func, true)
}



#[cfg(test)]
mod tests {
    use crate::tasks::task6and7;

    use super::*;


    #[test]
    fn test_fdnf() {
        let func = util::BooleanFunction::from("00010111").unwrap();
        let expr = get_fdnf(&func);
        //println!("{}", expr);

        let r = task6and7::check_dnf(func, &expr).unwrap();

        assert_eq!(r, true);
    }

    #[test]
    fn test_fknf() {
        let func = util::BooleanFunction::from("00010111").unwrap();
        let expr = get_fknf(&func);
        //println!("{}", expr);

        let r = task6and7::check_knf(func, &expr).unwrap();

        assert_eq!(r, true);
    }

    #[test]
    fn test_random() {

        let func = util::BooleanFunction::with_count_args(4);
        let expr = get_fdnf(&func);

        let r = task6and7::check_dnf(func, &expr).unwrap();
        assert_eq!(r, true);


        let func = util::BooleanFunction::with_count_args(4);
        let expr = get_fknf(&func);

        let r = task6and7::check_knf(func, &expr).unwrap();
        assert_eq!(r, true);

    }


}