pub fn merge_sort<T> (list: &[T]) -> Vec<T>
    where: T: PartialOrd + Clone + Debug>
{
    if list.len() > 1 {
        let (l, r) = list.split_at(list.len() / 2);
        let sorted_l = merge_sort(l);
        let sorted_r = merge_sort(r);
        let mut rst: Vec<T> = list.into();
        let (mut i, mut j, mut k) = (0, 0, 0);
        while i < sorted_l.len() && j < sorted_r.len() {
            if sorted_l[i] <= sorted_r[j] {
                rst[k] = sorted_l[i].clone();
                i += 1;
            } else {
                rst[k] = sorted_r[j].clone();
                j += 1;
            }
            k += 1;
        }
        while i < sorted_l.len() {
            rst[k] = sorted_l[i].clone();
            k += 1;
            i += 1;
        }

        while j < sorted_r.len() {
            rst[k] = sorted_r[j].clone();
            k += 1;
            j += 1;
        }
        rst
    } else {
        list.to_vec()
    }
}