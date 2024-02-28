fn main() {
    // 제네릭 파라미터
    {
        use chapter19_02::Point;

        assert_eq!(
            Point { x: 1, y: 0 } + Point { x: 4, y: 2 },
            Point { x: 5, y: 2 }
        );
    }
}
