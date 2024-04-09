use rand::Rng;

/// Хранит валидное значение булевой функции
#[derive(Debug, Clone, PartialEq)]
pub struct BooleanFunction {
    func: String,
    count_arguments: i32,
}

impl BooleanFunction {
    fn check_valid_value(str: &String) -> bool {
        let mut ok = true;
        for i in str.chars() {
            ok &= i == '0' || i == '1';
        }
        ok
    }
    fn check_valid_size(len: usize) -> (bool, i32) {
        let mut r = 1usize;
        let mut count = 1;

        while r << 1 < len {
            r <<= 1;
            count += 1;
        }

        return (1usize << count == len, count);
    }
    
    pub fn from(str: impl Into<String>) -> Result<Self, &'static str> {
        let str = str.into();
        if !Self::check_valid_value(&str) {
            return Err("Invalid function value. Function must be consist by 0 or 1.")
        }
        if let (true, len) = Self::check_valid_size(str.len()) {
            Ok(BooleanFunction{func: str, count_arguments: len})
        } else {
            Err("Invalid function length. Function length must be a power of two.")
        }
    }

    /// Возвращает остаточную булевую функцию по номеру аргумента и его значению
    /// Индексация с 0 и идёт справо налево
    pub fn remainde_boolean_function(&self, num_arg: i32, value: bool) -> Result<String, &'static str> {
        // возвращает string :<
        // прикол в том, что булевая функция из одного аргумента вернет вектор функции длиной 1
        // это является невалидным значением для структуры util::BooleanFunction

        if num_arg >= self.count_arguments || num_arg < 0 {
            return Err("Argument number greater than maximum.");
        }

        let len = 1usize << num_arg;
        let mut string = String::new();

        for (i, ch) in self.func.chars().enumerate() {
            if (i / len & 1) as i32 == value as i32 {
                string.push(ch);
            }

        }
        
        Ok(string)
    }

    /// Возвращает случайную булевую функцию из n аргументов
    pub fn with_count_args(n: i32) -> Self {
        let n = std::cmp::min(n, 31);

        let mut string = String::new();
        for _ in 0..(1i32<<n) {
            let val = get_random(2);
            string.push(std::char::from_digit(val as u32, 10).unwrap());
        }

        BooleanFunction::from(string).unwrap()
    }


    pub fn get_count_args(&self) -> i32 {
        self.count_arguments
    }

    pub fn get_func(&self) -> &String {
        &self.func
    }

    pub fn as_vec_bool(&self) -> Vec<bool> {
        let mut vector = Vec::new();
        for i in self.func.chars() {
            vector.push(i == '1');
        }

        vector
    }

}

pub fn get_random(n: i32) -> i32 {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(0..n);

    random_number
}

/// Итератор по булевой функции
/// Возвращает вектор текущих значений аргументов функции и текущее значение функции
pub struct BooleanFunctionIterator<'a> {
    current: i32,
    cnt: usize,
    end: bool,
    func_it: std::str::Chars<'a>,
}

impl<'a> BooleanFunctionIterator<'a> {
    pub fn new(bool_func: &'a BooleanFunction) -> BooleanFunctionIterator<'a> {
        BooleanFunctionIterator{ current: 0, cnt: bool_func.count_arguments as usize, func_it: bool_func.func.chars(), end : false }
    }
}

impl<'a> Iterator for BooleanFunctionIterator<'a> {
    type Item = (Vec<bool>, bool);

    fn next(&mut self) -> Option<Self::Item> {
        if self.end {
            return None;
        }
        let mut ok = true;
        let mut vc = Vec::new();
        vc.resize(self.cnt, false);

        let mut len = 1;
        for j in (0..self.cnt).rev() {
            let value = self.current / len & 1;
            len *= 2;
            vc[j] = value == 1;
            ok &= vc[j];
        }
        self.end = ok;
        self.current += 1;
        Some((vc, self.func_it.next().unwrap() == '1'))
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_func() {
        let mut rng = rand::thread_rng();
        

        let mut ok = true;
        for _ in 1..50 {
            let random_number = rng.gen_range(0..5);
            ok &= random_number >= 0 && random_number <= 5;
        }
        assert_eq!(ok, true);
    }

    #[test]
    #[should_panic]
    fn check_struct_panic_value() {
        BooleanFunction::from("abc").unwrap();
    }

    #[test]
    #[should_panic]
    fn check_struct_panic_length() {
        BooleanFunction::from("1010101010").unwrap();
    }

    #[test]
    fn check_create_struct() {
        let r = BooleanFunction::from("10").unwrap();
        println!("{}", r.count_arguments);
    }

    #[test]
    fn test_iterator() {
        let func = "10100011";
        let ff = BooleanFunction::from(func).unwrap();

        let it =  BooleanFunctionIterator::new(&ff);
        
        for i in it {
            println!("{:?}", i);
        }

    }

}