fn main() {
    let mut vec = vec![1, 2, 3, 4, 5]; // инициализируем вектор
    println!("{:?}", vec);

    // Удаляем элемент с индексом 2
    let removed_element = vec.remove(2); // удаление происходит за O(n) и порядок элементов сохраняется
    println!("{:?}", vec);
    println!("{:?}", removed_element);

    let mut vec = vec![1, 2, 3, 4, 5];
    println!("{:?}", vec);

    // Удаляем элемент с индексом 2
    let removed_element = vec.swap_remove(2); // удаление происходит за O(1), порядок элементов не сохраняется
    println!("{:?}", vec);
    println!("{:?}", removed_element);
}
