#[derive(PartialEq, Debug)]
struct Shoe
{
    size: u32,
    style: String
}

fn shoes_in_my_size(my_size: u32, shoes: Vec<Shoe>) -> Vec<Shoe>
{
    shoes.into_iter().filter(|shoe| { shoe.size == my_size }).collect()
}

struct Counter
{
    count: u32
}

impl Counter
{
    fn new() -> Counter
    {
        Counter{ count : 0 }
    }
}
impl Iterator for Counter
{
    type Item = u32;

    fn next(& mut self) -> Option<Self::Item>
    {
        if self.count < 5
        {
            self.count += 1;
            Some(self.count)
        }
        else
        {
            None
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    #[test]
    fn iterator_demo()
    {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter(); // has iter has to be mutable
        
        assert_eq!(Some(&1), v1_iter.next());
        assert_eq!(Some(&2), v1_iter.next());
        assert_eq!(Some(&3), v1_iter.next());
        assert_eq!(None, v1_iter.next());
        assert_eq!(None, v1_iter.next());

        let total = v1.iter().sum();
        //println!("Did the consuming iterator Sum take ownership? {:?}", v1_iter);
        assert_eq!(6, total);
    }

    #[test]
    fn iterator_iterator_adaptor()
    {
        let v1 = vec![1, 2, 3];
        let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
        assert_eq!(vec![2, 3, 4], v2);
    }

    #[test]
    fn filter_by_size()
    {
        let shoes = vec![
            Shoe {
                style: String::from("boot"),
                size: 10
            },
            Shoe {
                style: String::from("sandal"),
                size: 13
            },
            Shoe {
                style: String::from("sneakers"),
                size: 10
            }
        ];

        let in_size = shoes_in_my_size(10, shoes);
        assert_eq!(in_size,
        vec![
            Shoe {
                style: String::from("boot"),
                size: 10
            },
            Shoe {
                style: String::from("sneakers"),
                size: 10
            }
        ]);
    }

    #[test]
    fn test_counter_iterator()
    {
        let mut counter = Counter::new();
        assert_eq!(Some(1), counter.next());
        assert_eq!(Some(2), counter.next());
        assert_eq!(Some(3), counter.next());
        assert_eq!(Some(4), counter.next());
        assert_eq!(Some(5), counter.next());
        assert_eq!(None, counter.next());
    }

    #[test]
    fn mash_up()
    {
        let sum : u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| { a * b})
            .filter(|x| {x%3 == 0})
            .sum();

        assert_eq!(18, sum);
    }
}