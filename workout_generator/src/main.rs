use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_defined_intensity = 10;
    let simulated_random_number = 7;

    println!("With function");
    generate_workout(simulated_user_defined_intensity, simulated_random_number);
    println!("With closure");
    generate_workout_closure(simulated_user_defined_intensity, simulated_random_number);
    println!("With cacher");
    generate_workout_cacher(simulated_user_defined_intensity, simulated_random_number);

    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    // println!("Can't use x here {:?}", x);
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}

fn simulated_expensive_calculation(intensity: u32) -> u32
{
    println!("Calculating slowly ...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32)
{
    let expensive_result = simulated_expensive_calculation(intensity);

    if intensity < 25
    {
        println!("Today do {} pushups", expensive_result);
        println!("Next, do {} pushups", expensive_result);
    }
    else
    {
        if random_number < 3
        {
            println!("Take a break today, remember to stay hydrated");
        }
        else
        {
            println!("Today run for {} minutes", expensive_result);
        }
    }
}

fn generate_workout_closure(intensity: u32, random_number: u32)
{
    let expensive_closure = |intensity| {
        println!("Calculating slowly ...");
        thread::sleep(Duration::from_secs(2));
        intensity
    };

    if intensity < 25
    {
        println!("Today do {} pushups", expensive_closure(intensity));
        println!("Next, do {} pushups", expensive_closure(intensity));
    }
    else
    {
        if random_number < 3
        {
            println!("Take a break today, remember to stay hydrated");
        }
        else
        {
            println!("Today run for {} minutes", expensive_closure(intensity));
        }
    }
}

fn generate_workout_cacher(intensity: u32, random_number: u32)
{
   let mut cacher = Cacher::new(|intensity| {
        println!("Calculating slowly ...");
        thread::sleep(Duration::from_secs(2));
        intensity
    });

    if intensity < 25
    {
        println!("Today do {} pushups", cacher.value(intensity));
        println!("Next, do {} pushups", cacher.value(intensity));
    }
    else
    {
        if random_number < 3
        {
            println!("Take a break today, remember to stay hydrated");
        }
        else
        {
            println!("Today run for {} minutes", cacher.value(intensity));
        }
    }
}

struct Cacher<T>
where T: Fn(u32) -> u32
{
    // limitation: needs a map of values or different parameters
    // calculation closure can be more generic on parameter types
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T> 
where T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T>
    {
        Cacher
        {
            calculation,
            value: None
        }
    }

    fn value(&mut self, arg: u32) -> u32
    {
        match self.value
        {
            Some(v) => v,
            None => 
            {
                let f = (self.calculation)(arg);
                self.value = Some(f);
                f
            }
        }
    }
}