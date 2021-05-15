

    const TEST_ARR: [i32; 10] = [55, 45, 34, 87, 12, -55, -1, 0, 100, 99];
    const SORTED: [i32; 10] = [-55, -1, 0, 12, 34, 45, 55, 87, 99, 100];
    #[test]
    fn test_bubble_sort() {
        let mut test_arr = TEST_ARR;
        assert_eq!(crate::bubble_sort(&mut test_arr), SORTED);
    }

    #[test]                                                               fn test_improved_bubble_sort() {
        let mut test_arr = TEST_ARR;
        assert_eq!(crate::i_bubble_sort(&mut test_arr), SORTED);            }

    #[test]                                                               fn test_selection_sort() {
        let mut test_arr = TEST_ARR;
        assert_eq!(crate::selection_sort(&mut test_arr), SORTED);            }

    #[test]                                                               fn test_insertion_sort() {
        let mut test_arr = TEST_ARR;
        assert_eq!(crate::insertion_sort(&mut test_arr), SORTED);            }

