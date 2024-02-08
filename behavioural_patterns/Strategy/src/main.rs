// Define the strategy interface
trait SortStrategy<T> {
    fn sort(&self, data: &mut [T]);
}

// Define concrete sorting strategies
struct BubbleSortStrategy;
struct QuickSortStrategy;

impl<T: Ord> SortStrategy<T> for BubbleSortStrategy {
    fn sort(&self, data: &mut [T]) {
        // Implement bubble sort algorithm
        // This is just a placeholder implementation
        data.sort();
        println!("Bubble sort strategy used");
    }
}

impl<T: Ord> SortStrategy<T> for QuickSortStrategy {
    fn sort(&self, data: &mut [T]) {
        // Implement quick sort algorithm
        // This is just a placeholder implementation
        data.sort();
        println!("Quick sort strategy used");
    }
}

// Context class
struct Sorter<T> {
    strategy: Box<dyn SortStrategy<T>>,
}

impl<T> Sorter<T> {
    fn new(strategy: Box<dyn SortStrategy<T>>) -> Self {
        Sorter { strategy }
    }

    fn sort(&self, data: &mut [T]) {
        self.strategy.sort(data);
    }
}

fn main() {
    let mut data = vec![5, 2, 7, 3, 1];
    
    // Create context objects with different strategies
    let bubble_sorter = Sorter::new(Box::new(BubbleSortStrategy));
    let quick_sorter = Sorter::new(Box::new(QuickSortStrategy));

    // Use bubble sort strategy
    bubble_sorter.sort(&mut data);
    println!("Sorted data using bubble sort: {:?}", data);

    // Use quick sort strategy
    quick_sorter.sort(&mut data);
    println!("Sorted data using quick sort: {:?}", data);
}
