# 867. Transpose Matrix
# https://leetcode.com/problems/transpose-matrix/

defmodule Solution do
  @spec transpose(matrix :: [[integer]]) :: [[integer]]
  def transpose(matrix) do
    matrix
    |> Enum.zip()
    |> Enum.map(fn tup -> Tuple.to_list(tup) end)
  end
end

defmodule Main do
  def main do
    test_cases = [
      %{
        matrix: [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
        expected: [[1, 4, 7], [2, 5, 8], [3, 6, 9]]
      },
      %{
        matrix: [[1, 2, 3], [4, 5, 6]],
        expected: [[1, 4], [2, 5], [3, 6]]
      }
    ]

    test_cases
    |> Enum.each(fn %{matrix: matrix, expected: expected} ->
      result = Solution.transpose(matrix)

      IO.puts("Input: matrix = #{inspect(matrix)}")

      IO.puts("Output: #{inspect(result)}")
      IO.puts("Expected: #{inspect(expected)}")
      IO.puts("Test #{if result == expected, do: "Passed", else: "Failed"}")
      IO.puts("------")
    end)
  end
end

Main.main()
