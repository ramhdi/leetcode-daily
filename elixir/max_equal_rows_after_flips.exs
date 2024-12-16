defmodule Solution do
  @spec max_equal_rows_after_flips(matrix :: [[integer]]) :: integer
  def max_equal_rows_after_flips(matrix) do
    matrix
    # Step 1: Transform each row into a pattern
    |> Enum.map(&pattern_from_row/1)
    # Step 2: Build the frequency map
    |> Enum.reduce(%{}, fn pattern, acc -> Map.update(acc, pattern, 1, &(&1 + 1)) end)
    |> Map.values()
    # Step 3: Find the maximum frequency
    |> Enum.max()
  end

  # Helper function to compute the pattern for a row
  defp pattern_from_row(row) do
    # Get the first element
    first = hd(row)
    # XOR each element with the first
    row
    |> Enum.map(&Bitwise.bxor(&1, first))
  end
end

defmodule Main do
  def main do
    test_cases = [
      %{matrix: [[0, 1], [1, 1]], expected: 1},
      %{matrix: [[0, 1], [1, 0]], expected: 2},
      %{matrix: [[0, 0, 0], [0, 0, 1], [1, 1, 0]], expected: 2}
    ]

    Enum.each(test_cases, fn %{matrix: matrix, expected: expected} ->
      result = Solution.max_equal_rows_after_flips(matrix)
      IO.puts("Input: matrix = #{inspect(matrix)}")
      IO.puts("Output: #{result}")
      IO.puts("Expected: #{expected}")
      IO.puts("Test #{if result == expected, do: "Passed", else: "Failed"}")
      IO.puts("------")
    end)
  end
end

Main.main()
