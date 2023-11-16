from array_operation import sort_array, reverse_array, append_array, insert_array


def test_sort_array():
    arr = [2, 5, 1, 4, 3]
    expected = [1, 2, 3, 4, 5]

    result = sort_array(arr)

    assert result == expected


def test_reverse_array():
    arr = [9, 8, 7, 6]
    expected = [6, 7, 8, 9]

    result = reverse_array(arr)

    assert result == expected


def test_append_array():
    arr = [9, 8, 7, 6]
    item = 5
    expected = [9, 8, 7, 6, 5]

    result = append_array(arr, item)

    assert result == expected


def test_insert_array():
    arr = [9, 7, 6]
    item = 8
    index = 1
    expected = [9, 8, 7, 6]

    result = insert_array(arr, index, item)

    assert result == expected
