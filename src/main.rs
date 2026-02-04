mod first;
mod model;
mod second;
mod third;

use first::say_hello;
use second::say_hello as say_hello_second;
#[test]
fn test_use() {
    say_hello();
    say_hello_second();
    crate::first::second::third::say_hello();
}

use model::*; // untuk mengambil semua member di modul, baik struct, type function dan lain-lain
// use model::{User, test}; // atau bisa juga pilih sebaigian saja
#[test]
fn test_module() {
    let user = User {
        first_name: String::from("Ibrohim"),
        middle_name: String::from("Hendra"),
        last_name: String::from("Sairony"),
        email: String::from("ibrohim@gmai.com"),
        age: 20,
    };

    user.say_hello("Budi");
}

fn main() {
    println!("Hello, world!");
}

#[test] // ! istilahnya : annotation

fn test_hello_word() {
    println!("Hellao")
}

#[test]
fn variable() {
    let name: &str = "Ibrohim Sairony";
    println!("Hello {}", name)
}

#[test]
fn mutable() {
    let mut name = "Sairony";
    println!("Hello {}", name);
    name = "Ibrohim Sairony";
    println!("Hello {}", name)
}

// #[test]
// fn static_typing() { // ! type data tidak bisa diubah
//     let mut name = "Sairony";
//     name = 10;
//     println!("Hello {}", name)
// }

#[test]
fn shadowing() {
    let name = "Sairony";
    println!("Hello {}", name);

    // ! variable seblumnya akan tertutupi
    let name = 10;
    println!("Hello {}", name)
}

#[test]
fn type_data() {
    // ! secara umum type data ada 2 scalar dan compound
    // * scalar yang datanya satu.
    // * compound seperti array dan tuple
    let age: i32 = 20;
    println!("Age {}", age)
}

#[test]
fn explicit_type_data() {
    let age: i32 = 20;
    println!("Age {}", age)
}

#[test]
fn number() {
    let a: i8 = 10;
    println!("Age {}", a);

    let b: f32 = 10.4;
    println!("float {}", b)
}

#[test]
fn number_confersion() {
    let a: i8 = 10;
    let b: i32 = a as i32;
    println!("{}", b);
    let c: i64 = b as i64;
    println!("{}", c);

    // ! overflow confersion
    let d: i64 = 100000000000000;
    let f: i8 = d as i8;
    println!("{}", f);
}

#[test]
fn number_operator() {
    let a = 10;
    let b = 30;
    let c = a % b;
    println!("{}", c);
    let d = a / b;
    println!("{}", d);
    let f = a * b;
    println!("{}", f);
    let g = a + b;
    println!("{}", g);
    let h = a - b;
    println!("{}", h);
}

#[test]
fn augmented_assignment() {
    let mut a = 10;
    println!("{}", a);

    a += 10;
    println!("{}", a);

    a -= 10;
    println!("{}", a);

    a *= 10;
    println!("{}", a);
}

#[test]
fn boolean() {
    let a = true;
    let b = false;
    println!("{} , {}", a, b)
}

#[test]
fn comparison() {
    // <  >  <=  >=  ==  !=
    let result = 30 > 110;
    println!("{}", result);
}

#[test]
fn operator_boolean() {
    // && || !
    let result = true || false;
    println!("{}", result);
}

#[test]
fn char_type() {
    // ! pakai petik ''
    let char1 = 'a';
    let char2 = 'b';
    println!("{}, {}", char1, char2);
}

#[test]
fn tuple() {
    let data = (10, 10.5, true); //  * bisa beda tipe data
    println!("{:?}", data);
    // * {:?}  artinya : debug information

    // let a = data.0;
    // let b = data.1;
    // let b = data.2;

    let (a, b, c) = data;
    println!("{}, {}, {}", a, b, c);

    let (f, g, _) = data; // * tanda _ untuk mencuekin
    println!("{}, {}", f, g);
}

#[test]
fn mutable_tuple() {
    let mut data = (10, 10.5, true);
    println!("{:?}", data);

    data.0 = 40;
    data.1 = 20.4;
    data.2 = false;
    println!("{:?}", data);
}

// ! Pembahasan unit. Unit adalah tuple kosong ()
fn unit() {
    // return value tuple kosong
    println!("Hello!")
}
#[test]
fn test_unit() {
    let result: () = unit(); // result adalah tuple kosong / unit ()
    println!("{:?}", result);

    let test_unit: () = (); // langsung membuat unit
    println!("{:?}", test_unit);

    //unit jarang digunakan
}
#[test]
fn array() {
    let array: [i32; 5] = [1, 2, 3, 4, 5]; //  * fix tipe data
    println!("{:?}", array);

    let a = array[0]; // * pakai [] kalau tuple pakai .
    let b = array[1];
    println!("{}, {}", a, b);
}

#[test]
fn mutable_array() {
    let mut array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", array);

    array[0] = 10;
    array[1] = 20;
    println!("{:?}", array);
}

#[test]
fn length_array() {
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", array);

    let panjang: usize = array.len();
    println!("{}", panjang);
}

#[test]
fn two_dimensional_array() {
    let matrix = [[1, 2], [1, 2]]; // bisa bertambah array didalam array lagi

    println!("{:?}", matrix);
    println!("{:?}", matrix[0]);
    println!("{:?}", matrix[1]);
    println!("{}", matrix[0][0]);
    println!("{}", matrix[0][1]);
}

const MINIMUM: i32 = 0;
#[test]
fn constant() {
    const MAXIMUM: i32 = 100;
    println!("{} {}", MAXIMUM, MINIMUM);
}

#[test]
fn variable_scope() {
    println!("{}", MINIMUM); // global_scope

    let ibrohim = 0; // variable_scope

    {
        // inner_scope
        println!("{}", ibrohim); // success
        let _sairony = 1;
    }
    // println!("{}", sairony); // * error - outter_scope
}

#[test]
fn stack_heap() {
    // Manajement Memory ada 2 : stack dan heap
    function_a();
    function_b()
}

fn function_a() {
    let a = 10;
    let b = String::from("Ibrohim");
    println!("{}, {}", a, b)
}

fn function_b() {
    let a = 10;
    let b = String::from("Sairony");
    println!("{}, {}", a, b);
}

#[test]
fn string_slice() {
    // string_slice beda dengan string biasa
    // kode type_data : &str
    let name = "   Ibrohim Sairony    ";
    let trim = name.trim();
    println!("{}", name);
    println!("{}", trim);

    let mut _username = "ibrohim"; // fixed-size.  gk bisa diapa-apakan. selalu ada di memory.
    _username = "Sairony"; // data "ibrohim" gk hilang. cuma nama variable diganti.
    println!("{}", _username);

    // number juga sama seperti string_slice : fixed-size
    let mut _value = 10;
    _value = 20 // ! aku baru tau perilaku seprti ini
}

#[test]
fn string() {
    let mut name = String::from("Ibrohim");
    println!("{}", name);

    name.push_str(" Sairony");
    println!("{}", name);

    let saad = name.replace("Ibrohim", "Saad");
    println!("{}", saad);
}

#[test]
fn data_copy() {
    // untuk data di stack (fixed-size) implement copy, jadi variable / owner lama bisa diakses
    let a = 10;
    let b = a;
    println!("{}, {}", a, b);
}

#[test]
fn ownership_movement() {
    // untuk data di heap (data-bisa-berkembang) itu bukan copy melainkan move jadi owner lama sudah hilang
    let name1 = String::from("Ibrohim");
    println!("{}", name1);

    let name2 = name1; //ownership pindah ke name2
    println!("{}", name2);
    // println!("{}", name1); // ! error karena ownership sudah berpindah
}

#[test]
fn clone() {
    // solusi untuk copy data heap. bukan movement_ownership
    let name1 = String::from("Ibrohim");
    let name2 = name1.clone();
    println!("{}, {}", name1, name2)
}

#[test]
fn if_expression() {
    let value = 8;
    let result: &str;

    if value >= 8 {
        result = "Good";
    } else if value >= 6 {
        result = "Not bad";
    } else if value >= 3 {
        result = "Bad";
    } else {
        result = "Very Bad";
    }
    println!("{}", result);
}

#[test]
fn if_in_let_statement() {
    // return value dari if statement
    let value = 9;
    let result = if value >= 8 {
        // if digabung dengan let dekralasi
        "Good"
    } else if value >= 6 {
        "Not bad"
    } else if value >= 3 {
        "Bad"
    } else {
        "Very Bad"
    };

    println!("{}", result);
}

#[test]
fn loop_expression() {
    let mut counter = 0;

    loop {
        counter += 1;

        if counter > 10 {
            break;
        }

        if counter % 2 != 0 {
            continue;
        }

        println!("Counter : {}", counter);
    }
}

#[test]
fn loop_return_value() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter > 10 {
            break counter * 2;
        }
    };
    println!("Result : {}", result);
}

#[test]
fn loop_label() {
    let mut number = 1;
    'outer: loop {
        // buat dihentikan di dalam loop lain
        let mut i = 1;
        loop {
            if number > 10 {
                break 'outer;
            }

            println!("{} * {} = {}", number, i, number * i);
            i += 1;

            if i > 10 {
                break;
            }
        }
        number += 1;
    }
}

#[test]
fn while_loop() {
    let mut counter = 0;
    while counter >= 0 {
        if counter % 2 == 0 {
            println!("Counter : {}", counter);
        }
        counter += 1;
    }
}

#[test]
fn array_iteration() {
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];
    let mut index = 0;

    while index < array.len() {
        println!("Value : {}", array[index]);
        index += 1;
    }
}

#[test]
fn array_iteration_for() {
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];
    for value in array {
        println!("Value : {}", value);
    }
}

#[test]
fn range() {
    // range adalah type data collection mirip array.
    // defaultnya adalah exclusive, nilai akhir gk diambil

    let range = 0..5; // saat dilooping hasilnya 1 - 4.  5 tidak dianggap.
    println!("Start : {}", range.start); // Inclusive : diambil nilai awal
    println!("End : {}", range.end); // 5 // Exclusive : nilai paling akhir tidak diambil saat looping

    let array: [&str; 5] = ["A", "B", "C", "D", "E"];
    for i in range {
        println!("Value : {}", array[i]);
    }

    // bisa juga langsung
    for i in 0..5 {
        println!("Value : {}", array[i]);
    }
}

#[test]
fn range_inxlucive() {
    let range = 0..=4; // diakhiri =    yang terakhir akan diambil
    println!("Start : {}", range.start());
    println!("End : {}", range.end());

    let array: [&str; 5] = ["A", "B", "C", "D", "E"];
    for i in range {
        println!("Value : {}", array[i]);
    }
}

fn say_goodbye(firs_name: &str, last_name: &str) {
    println!("GoodBye {} {}", firs_name, last_name);
}

#[test]

fn function_parameter() {
    say_goodbye("Ibrohim", "Sairony");
    say_goodbye("Ahmad", "Sairony");
}

fn factorial_loop(n: i32) -> i32 {
    if n < 1 {
        return 0;
    }
    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }
    result
}

#[test]
fn test_factorial_loop() {
    let result = factorial_loop(10);
    println!("{}", result);

    let result = factorial_loop(-30);
    println!("{}", result);
}

fn factorial_recursive(n: i32) -> i32 {
    if n < 1 {
        return 0;
    }

    n * factorial_loop(n - 1)
}

fn print_text(value: String, times: u32) {
    if times == 0 {
        return;
    } else {
        println!("Recursive ke - {} nilainya : {}", times, value);
    }
    print_text(value, times - 1);
}

#[test]
fn test_print_test() {
    print_text(String::from("Ibrohim"), 20);
}

#[test]
fn test_factorial_recursive() {
    let result = factorial_recursive(5);
    println!("{}", result);
}

fn print_number(n: i32) {
    println!("{}", n);
}

fn hi(name: String) {
    println!("name : {}", name);
}

#[test]
fn test_hi() {
    // ! type data yang ada di heap, jika dikirim ke argument/parameter, ownership akan berganti
    let number = 10;
    print_number(number);
    println!("{}", number); // berhasil; karena stack tidak mengenal transfer ownership

    let name = String::from("Ibrohim");
    hi(name);
    // println!("{}", name); // error karena ownership sudah pindah
}

fn full_name(first_name: &String, last_name: &String) -> String {
    // & untuk membuat reference. jadi yang dikirim datanya saja bukan ownernya
    format!("{} {}", first_name, last_name)
}

#[test]
fn test_full_name() {
    let first_name = String::from("Ibrohim");
    let last_name = String::from("Sairony");

    let full_name = full_name(&first_name, &last_name);

    println!("{}", full_name);
    println!("{}", first_name);
    println!("{}", last_name);
}

fn change_value(_value: &String) {
    // value.push_str("Test"); // error karena borrow sebagai immutable bu8kan mutable
}

#[test]
fn test_change_value() {
    let value = String::from("Ibrohim");
    change_value(&value);
    println!("{}", value);
}

fn change_value_mutable(value: &mut String) {
    value.push_str("Test");
}

#[test]
fn test_change_value_mutable() {
    // dalam satu waktu / lifetime hanya boleh ada satu ref_mutable. dan gk boleh ada lagi walau ref_imutable
    let mut value = String::from("Ibrohim");

    let value_boorow1 = &value; // tidak error karena proses peminjaman mutable heap setelah siklus ini. kalau dipindah ke bawah error.
    println!("{}", value_boorow1);
    let value_boorow2 = &mut value;

    //change_value_mutable(&mut value); // error karena ref_mutable sudah dipakai di variable atas. ini melanggar lifetime. dalam satu siklus hidup hanya boleh satu ref/borrow_mutable dalam heap
    change_value_mutable(value_boorow2);
    println!("{}", value);
}

fn get_full_name(first_name: &String, last_name: &String) -> String {
    // gk bisa -> &String seperti di golang, karena akan menunjuk/ ref ke data yang sudah hilang.
    let name = format!("{} {}", first_name, last_name);
    name
}

#[test]
fn test_get_full_name() {
    let first_name = String::from("Ibrohim");
    let last_name = String::from("Sairony");

    let full_name = get_full_name(&first_name, &last_name);

    println!("{}", full_name);
}

#[test]
fn slice_reference() {
    let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let slice1 = &array[..];
    println!("{:?}", slice1);

    let slice2 = &array[0..5];
    println!("{:?}", slice2);

    let slice3 = &array[5..];
    println!("{:?}", slice3);
}

#[test]
fn string_slic() {
    let name = String::from("Ibrohim Sairony");
    println!("{}", name);

    let first_name = &name[0..7];
    println!("{}", first_name);
    let last_name = &name[8..];
    println!("{}", last_name);
}

// ? Materi Tentang Struct

struct Person {
    first_name: String,
    middle_name: String,
    last_name: String,
    age: u8,
}

impl Person {
    fn say_hello(&self, name: &str) {
        // ini method karena ada &self, bukan associated function
        println!("Hello {}, my name is {}", name, self.first_name)
    }
}

fn print_person(person: &Person) {
    println!("{}", person.first_name);
    println!("{}", person.middle_name);
    println!("{}", person.last_name);
    println!("{}", person.age);
}

#[test]
fn test_struct_person() {
    let first_name = String::from("Ibrohim");
    let last_name = String::from("Sairony");

    let person = Person {
        age: 21,
        first_name, // istilahnya : init sorthand // ! Hati-hati : Perpindahan ownership
        middle_name: String::from("Hendra"),
        last_name,
    };
    print_person(&person);

    let person2 = Person {
        first_name: person.first_name.clone(),
        last_name: person.last_name.clone(), // solusi untuk mencegah perpindahan ownership
        middle_name: person.middle_name.clone(),
        ..person // istilah : Update Syntax (untuk copy semua data) artinya akan terjadi perpindahan ownership jika datanya heap
    };

    print_person(&person2);
}

struct GeoPoint(f64, f64);

impl GeoPoint {
    fn new(long: f64, lat: f64) -> GeoPoint {
        // ini adalah associated function bukan method karena tidak ada self.
        GeoPoint(long, lat)
    }
}

#[test]
fn test_associated_function() {
    let geo_point = GeoPoint::new(10.2234, 40.32423); // cara akses pakai :: bukan . seperti method
    println!("{}", geo_point.0);
    println!("{}", geo_point.1);
}

#[test]
fn tuple_struct() {
    let geo_point = GeoPoint(-6.233223, 100.2132432);
    println!("{}", geo_point.0);
    println!("{}", geo_point.1);
}

struct Nothing;

#[test]
fn test_nothing() {
    let _nothing1 = Nothing;
    let _nothing2 = Nothing;
}

#[test]
fn test_method() {
    let person = Person {
        first_name: String::from("Ibrohim"),
        middle_name: String::from("Hendra"),
        last_name: String::from("Sairony"),
        age: 20,
    };

    person.say_hello("Ahmad");
}

enum Level {
    Reguler,
    Premium,
    Platinum,
}

#[test]
fn test_enum() {
    let level = Level::Reguler;
    match level {
        Level::Reguler => {
            println!("Reguler");
        }
        Level::Premium => {
            println!("Premium");
        }
        Level::Platinum => {
            println!("Platinum");
        }
    }
    let _level1 = Level::Premium;
    let _level2 = Level::Platinum;
}

enum Payment {
    // Enum bisa juga digunakan untuk menyimpan data layaknya Tuple.
    CreditCard(String),
    BankTransfer(String, String),
    EWallet(String, String),
}

impl Payment {
    fn pay(&self, amount: u32) {
        match self {
            Self::CreditCard(number) => {
                println!("Paying with credit card {} amount {}", number, amount)
            }
            Self::BankTransfer(bank, number) => {
                println!(
                    "Paying with transfer bank {} {} amount {}",
                    bank, number, amount
                )
            }
            Payment::EWallet(wallet, number) => {
                // Bisa pakai Self:: atau Payment::
                println!(
                    "Paying with e-wallet {} {} amount {}",
                    wallet, number, amount
                )
            }
        }
    }
}

#[test]
fn test_paymentt() {
    let _payment1 = Payment::CreditCard(String::from("23489324"));
    _payment1.pay(3000000);
    let _payment2 = Payment::BankTransfer(String::from("BCA"), String::from("2348092"));
    _payment2.pay(3040000);
    let _payment3 = Payment::EWallet(String::from("Gopay"), String::from("2348092"));
    _payment3.pay(23440000);
}

#[test]
fn test_match_value() {
    // Jarang digunakan karena lebih simple pakai if else
    let name = "Joko";
    match name {
        "Joko" => {
            println!("Hello {}", name);
        }
        "Budi" => {
            println!("Hello {}", name);
        }
        _else => {
            println!("Hello {}", _else); // bisa juga pakai varablenya
        }
    }
    match name {
        "Budi" | "Joko" => {
            // Namanya Multiple Pattern pakai pipe |
            println!("Hallo Bos")
        }
        "Otong" | "Ucup" => {
            println!("Hallo Om")
        }
        other => {
            println!("Hallo {}", other);
        }
    }
}

#[test]
fn test_range_pattern() {
    let value = 10;
    match value {
        75..=100 => {
            println!("Great!")
        }
        50..=74 => {
            println!("Good")
        }
        25..=49 => {
            println!("Not Bad")
        }
        0..=24 => {
            println!("Bad")
        }
        other => {
            println!("invalid value {}", other)
        }
    }
}

#[test]
fn test_struct_match_pattern() {
    let geo_point = GeoPoint::new(0.0, 13.0);
    // Untuk yang stuct tanpa field / struct tuple
    match geo_point {
        GeoPoint(long, 0.0) => {
            println!("Long : {}", long);
        }
        GeoPoint(0.0, lat) => {
            println!("Lat : {}", lat);
        }
        GeoPoint(long, lat) => {
            println!("Long : {}, Lat : {}", long, lat);
        }
    }
    // Untuk yang stuct dengan field / struct biasa

    let person = Person {
        first_name: String::from("Ibrohim"),
        middle_name: String::from("Hendra"),
        last_name: String::from("Sairony"),
        age: 20,
    };

    match person {
        Person {
            first_name,
            last_name,
            ..
        } => {
            println!("{} {}", first_name, last_name);
        }
    };
}

#[test]
fn test_ingnoring() {
    // Menggunakan _
    let geo_point = GeoPoint::new(0.0, 13.0);
    match geo_point {
        GeoPoint(_, lat) => {
            println!("Lat : {}", lat);
        }
    }
}

#[test]
fn test_ignoring_range() {
    let value = 10;
    match value {
        75..=100 => {
            println!("Great!")
        }
        50..=74 => {
            println!("Good")
        }
        25..=49 => {
            println!("Not Bad")
        }
        0..=24 => {
            println!("Bad")
        }
        _ => {
            // Menggunakan _
            println!("invalid value")
        }
    }
}

#[test]
fn test_match_expression() {
    let value = 2;
    let result = match value {
        0 => "nol",
        1 => "satu",
        2 => "dua",
        _ => "invalid value",
    };
    println!("{}", result);
}

type Age = u8; // type alias untu kembuat type data baru
type IdentityNumber = String;

struct Customer {
    id: IdentityNumber,
    name: String,
    age: Age,
}

type Pelanggan = Customer; // type alias juga bisa untuk struct

#[test]
fn test_type_alias() {
    let customer = Customer {
        id: String::from("12324234"),
        age: 20,
        name: String::from("Ibrohim"),
    };

    let pelanggan = Pelanggan {
        id: String::from("12324234"),
        age: 20,
        name: String::from("Ibrohim"),
    };
    println!("{} {} {}", customer.id, customer.name, customer.age);
    println!("{} {} {}", pelanggan.id, pelanggan.name, pelanggan.age);
}

trait CanSayHello {
    fn hello(&self) -> String {
        // Default Implementation
        String::from("Hello")
    }
    fn say_hello(&self) -> String;
    fn say_hello_to(&self, name: &str) -> String;
}

impl CanSayHello for Person {
    fn say_hello(&self) -> String {
        format!("Hello, my name is {}", self.first_name)
    }
    fn say_hello_to(&self, name: &str) -> String {
        format!("Hello {}, my name is {}", name, self.first_name)
    }
}

trait CanSayGoodBye {
    fn good_bye(&self) -> String;
    fn good_bye_to(&self, name: &str) -> String;
}

impl CanSayGoodBye for Person {
    fn good_bye(&self) -> String {
        format!("Good Bye, my name is {}", self.first_name)
    }
    fn good_bye_to(&self, name: &str) -> String {
        format!("Good Bye {}, my name is {}", name, self.first_name)
    }
}

fn say_hello_trait(value: &impl CanSayHello) {
    // trait in paramater
    println!("{}", value.say_hello());
}

fn hello_and_goodbye(value: &(impl CanSayHello + CanSayGoodBye)) {
    println!("{}", value.say_hello());
    println!("{}", value.good_bye());
}

#[test]
fn test_trait() {
    let person = Person {
        first_name: String::from("Ibrohim"),
        middle_name: String::from("Hendra"),
        last_name: String::from("Sairony"),
        age: 20,
    };

    say_hello_trait(&person);
    hello_and_goodbye(&person);

    let say = person.say_hello_to("Budi");
    println!("{}", say);
    println!("{}", person.hello());
    println!("{}", person.good_bye());

    println!("{}", CanSayHello::say_hello(&person));
    person.say_hello("Budi");
}

struct SimplePerson {
    name: String,
}

impl CanSayGoodBye for SimplePerson {
    fn good_bye(&self) -> String {
        format!("Good bye, my name is {}", self.name)
    }
    fn good_bye_to(&self, name: &str) -> String {
        format!("Good bye {}, my name is {}", name, self.name)
    }
}

fn create_person(name: String) -> impl CanSayGoodBye {
    SimplePerson { name }
}

#[test]
fn test_return_trait() {
    let person = create_person(String::from("Ibrohim"));
    println!("{}", person.good_bye());
    println!("{}", person.good_bye_to("Budiy"));
}

trait CanSay: CanSayHello + CanSayGoodBye {
    // mirip konsep inheritance / pewarisan
    fn say(&self) {
        println!("{}", self.say_hello());
        println!("{}", self.good_bye());
    }
}

struct SimpleMan {
    name: String,
}

impl CanSay for SimpleMan {}

impl CanSayHello for SimpleMan {
    fn say_hello(&self) -> String {
        format!("Hello, my name is {}", self.name)
    }
    fn say_hello_to(&self, name: &str) -> String {
        format!("Hello {}, my name is {}", name, self.name)
    }
}
impl CanSayGoodBye for SimpleMan {
    fn good_bye(&self) -> String {
        format!("Hello, my name is {}", self.name)
    }
    fn good_bye_to(&self, name: &str) -> String {
        format!("Hello {}, my name is {}", name, self.name)
    }
}

struct Point<T = i32> {
    // T = i32 itu untuk default generic type
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
    fn get_y(&self) -> &T {
        &self.y
    }
}

#[test]
fn test_generic_struct() {
    let integer = Point::<i32> { x: 20, y: 30 };
    let float = Point { x: 20.3, y: 101.0 }; // ! bisa juga gk perlu ::<>
    // let float = Point::<f64> { x: 20.3, y: 101.0 };
    println!("{}, {}", integer.x, integer.y);
    println!("{}, {}", float.x, float.y);
}

enum Value<T> {
    NONE,
    VALUE(T),
}

#[test]
fn test_generic_enum() {
    let value = Value::<i32>::VALUE(10); // bisa gk pakai ::<>

    match value {
        Value::NONE => {
            println!("none")
        }
        Value::VALUE(value) => {
            println!("Value : {}", value);
        }
    }
}

struct Hi<T = SimplePerson>
where
    T: CanSayGoodBye,
{
    // bisa juga CanSayGoodBye + CanSayHello // ! Istilahnya type bound
    value: T,
}

#[test]
fn test_generic_bound() {
    let hi = Hi::<SimplePerson> {
        value: SimplePerson {
            name: String::from("Ibrohim"),
        },
    };
    println!("{}", hi.value.good_bye());
}

fn min<T>(value1: T, value2: T) -> T
where
    T: PartialOrd,
{
    if value1 > value2 { value2 } else { value1 }
}

#[test]
fn generic_in_function() {
    let result = min(10, 44);
    println!("{}", result);
}

#[test]
fn test_generic_method() {
    let point = Point { x: 10, y: 20 };
    println!("{}", point.get_x());
    println!("{}", point.get_y());
    println!("{}", point.get_value());
}

trait GetValue<T>
where
    T: PartialOrd,
{
    fn get_value(&self) -> &T;
}

impl<T> GetValue<T> for Point<T>
where
    T: PartialOrd,
{
    fn get_value(&self) -> &T {
        &self.x
    }
}

struct Apple {
    quantity: i32,
}
use core::ops::{Add, Sub};
use std::{
    cmp::Ordering,
    collections::{LinkedList, VecDeque},
    fmt::Debug,
};
impl<'a, 'b> Add<&'b Apple> for &'a Apple {
    type Output = Apple;

    fn add(self, rhs: &'b Apple) -> Apple {
        Apple {
            quantity: self.quantity + rhs.quantity,
        }
    }
}
impl<'a, 'b> Sub<&'b Apple> for &'a Apple {
    type Output = Apple;

    fn sub(self, rhs: &'b Apple) -> Apple {
        Apple {
            quantity: self.quantity - rhs.quantity,
        }
    }
}

#[test]
fn test_operator_add_overloading() {
    let apple1 = Apple { quantity: 10 };
    let apple2 = Apple { quantity: 10 };

    let apple3 = &apple1 + &apple2; // ini keren. harusnya error. tapi operatornya sudah dimodif
    println!("{}", apple3.quantity);

    let apple4 = &apple3 - &apple1;
    println!("{}", apple4.quantity);
}

// Optional Value // ! Karena di rust gk ada null , nil dan undefined. Jadi solusi pakai optional enum
fn double(n: Option<i32>) -> Option<i32> {
    match n {
        None => None,
        Some(i) => Some(i * 2),
    }
}

//
fn bagi<T>(n: Option<T>) -> Option<T>
where
    T: std::ops::Div<Output = T> + From<u8>,
{
    match n {
        None => None,
        Some(i) => Some(i / T::from(2)),
    }
}

#[test]
fn test_option() {
    let result = double(None);
    println!("{:?}", result);

    let result = double(Some(43));
    println!("{:?}", result);

    let result = bagi(Some(43.4));
    println!("{:?}", result);
}

impl PartialEq for Apple {
    fn eq(&self, other: &Self) -> bool {
        self.quantity == other.quantity
    }
}

impl PartialOrd for Apple {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // self.quantity.partial_cmp(&other.quantity) // ! menggunakan method yang sudah tersedia di type integer.

        // ! Cara manual
        if self.quantity > other.quantity {
            Some(Ordering::Greater)
        } else if self.quantity < other.quantity {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}

#[test]
fn test_comparing() {
    let apple1 = Apple { quantity: 23 };
    let apple2 = Apple { quantity: 30 };

    println!("Apple1 == Apple2 : {}", apple1 == apple2);
    println!("Apple1 != Apple2 : {}", apple1 != apple2);
    println!("Apple1 > Apple2 : {}", apple1 > apple2);
    println!("Apple1 < Apple2 : {}", apple1 < apple2);
}

#[test]
fn test_string_manipulation() {
    let s = String::from("Ibrohim Sairony");

    println!("{}", s.to_uppercase());
    println!("{}", s.to_lowercase());
    println!("{}", s.len());
    println!("{}", s.replace("Ibrohim", "Ahmad"));
    println!("{}", s.contains("Ibrohim"));
    println!("{}", s.starts_with("Ibrohim"));
    println!("{}", s.ends_with("Ibrohim"));
    println!("{}", s.trim());
    println!("{}", &s[0..7]);
    println!("{:?}", s.get(0..7));
}

struct Category {
    id: String,
    name: String,
    price: u16,
}
impl Debug for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Category")
            .field("id", &self.id)
            .field("name", &self.name)
            // .field("price", &self.price) // ! Bisa di sembunyikan informasi saat debug untuk menjaga data sensitif
            .finish()
    }
}

#[test]
fn test_format() {
    let category = Category {
        id: String::from("GADGET"),
        name: String::from("Gadget"),
        price: 89,
    };

    println!("{:?}", category);
}

#[test]
fn test_closure() {
    // closure adalah function yang tidak memiliki nama / istilah umum : //! anonimus function
    let sum = |value1: i32, value2: i32| value1 + value2; // type return value boleh dihilangkan

    let result = sum(43, 23);
    println!("{}", result);
}

fn print_with_format(value: String, format: fn(String) -> String) {
    let result = format(value);
    println!("{}", result);
}

#[test]
fn test_closure_as_parameter() {
    print_with_format(String::from("Ibrohim"), |value| value.to_uppercase());
}

fn to_uppercase(value: String) -> String {
    value.to_uppercase()
}

#[test]
fn test_function_as_closure() {
    print_with_format(String::from("Ibrohim"), to_uppercase);
}

#[test]
fn test_closure_scope() {
    let mut counter = 0;
    let mut increment = || {
        counter += 1;
        println!("Increment");
    };
    increment();
    increment();
    increment();

    println!("Counter : {}", counter);
}

struct Counter {
    counter: i32,
}

impl Counter {
    fn increment(&mut self) {
        self.counter += 1;
        println!("Increment");
    }
}

#[test]
fn test_counter() {
    let mut counter = Counter { counter: 0 };
    counter.increment();
    counter.increment();
    counter.increment();
    println!("Counter : {}", counter.counter);
}

#[test]
fn test_vector() {
    // let mut names = Vec::<String>::new(); // bisa seperti ini tapi repot
    let mut names = vec![];
    names.push(String::from("Ibrohim"));
    names.push(String::from("Hendra"));
    names.push(String::from("Sairony"));

    for name in &names {
        println!("{}", name);
    }

    println!("{:?}", names);
}

#[test]
fn test_vector_deque() {
    // vector deque bisa tambah data dari depan.
    let mut names = VecDeque::new();
    names.push_back(String::from("Ibrohim"));
    names.push_back(String::from("Hendra"));
    names.push_front(String::from("Sairony"));

    for name in &names {
        println!("{}", name);
    }
    println!("{:?}", names);
    println!("{:?}", names[1]);
}

#[test]
fn test_linked_list() {
    // linked list itu lebih efissien. kelemahannya dia tidak bisa diakses pakai index. karena dia buta
    let mut names = LinkedList::new();
    names.push_back(String::from("Ibrohim"));
    names.push_back(String::from("Hendra"));
    names.push_front(String::from("Sairony"));

    for name in &names {
        println!("{}", name);
    }
    println!("{:?}", names);
    // println!("{:?}", names[1]); // error.. linked list gk bisa pakai index
}
