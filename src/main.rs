fn base_bubble_sort(vec : &mut Vec<i32>) {
    for i in 0..vec.len() {
        for j in 0..vec.len() - 1 - i {
            if vec[j] > vec[j + 1] {
                vec[j] = vec[j] ^ vec[j + 1];
                vec[j + 1] = vec[j] ^ vec[j + 1];
                vec[j] = vec[j] ^ vec[j + 1];
            }
        }
    }
}

fn template_bubble_sort<T: PartialOrd + Copy>(list : &mut Vec<T>) -> &Vec<T> {

    for i in 0..list.len() {
        for j in 0..list.len() - 1 {
            if list[j] > list[j + 1] {
                list.swap(j,j + 1);
            }
        }
    }
    list
}


fn main() {
    let mut list1 = vec![1,8,3,2,4,5,67,6];
    base_bubble_sort(&mut list1);
    println!("{:?} ",list1);

    let mut list2 = vec![1,14,63,42];
    template_bubble_sort(&mut list2);
    println!("{:?} ",list2);

    let mut list3 = vec!['A','G','B'];
    template_bubble_sort(&mut list3);
    println!("{:?} ",list3);
}
