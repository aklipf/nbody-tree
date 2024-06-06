#[doc(hidden)]
#[macro_export]
macro_rules! generate_bottomup {
    ($comments:tt) => {
        doc_comment! {
            concat!$comments,
            #[inline]
            pub fn bottomup<D>(&self, query: &[A; K], dist: A) -> Vec<NearestNeighbour<A, T>>
            where
                D: DistanceMetric<A, K>,
            {
                let mut matching_items = self.within_unsorted::<D>(query, dist);
                matching_items.sort();
                matching_items
            }
        }
    };
}
