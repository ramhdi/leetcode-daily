# https://leetcode.com/problems/max-chunks-to-make-sorted/
# 2024/12/19

# Iterative: 0ms, 73MB
# defmodule Solution do
#   @spec max_chunks_to_sorted(arr :: [integer]) :: integer
#   def max_chunks_to_sorted(arr) do
#     arr
#     |> Enum.with_index()
#     |> Enum.reduce({0, 0}, fn {num, i}, {chunks, max_num} ->
#       max_num = Kernel.max(max_num, num)

#       chunks =
#         if max_num == i do
#           chunks + 1
#         else
#           chunks
#         end

#       {chunks, max_num}
#     end)
#     |> Kernel.elem(0)
#   end
# end

# Tail recursive: 0ms, 72.7MB
defmodule Solution do
  @spec max_chunks_to_sorted(arr :: [integer]) :: integer
  def max_chunks_to_sorted(arr) do
    do_max_chunks_to_sorted(arr, 0, 0, 0)
  end

  defp do_max_chunks_to_sorted([], _, chunks, _), do: chunks

  defp do_max_chunks_to_sorted([num | rest], index, chunks, max_num) do
    max_num = max(max_num, num)

    chunks =
      if max_num == index do
        chunks + 1
      else
        chunks
      end

    do_max_chunks_to_sorted(rest, index + 1, chunks, max_num)
  end
end

defmodule Main do
  def main do
    test_cases = [
      %{
        arr: [4, 3, 2, 1, 0],
        expected: 1
      },
      %{
        arr: [1, 0, 2, 3, 4],
        expected: 4
      }
    ]

    test_cases
    |> Enum.each(fn %{arr: arr, expected: expected} ->
      result = Solution.max_chunks_to_sorted(arr)
      IO.puts("Input: arr = #{inspect(arr)}")
      IO.puts("Output: #{inspect(result)}")
      IO.puts("Expected: #{inspect(expected)}")
      IO.puts("Test #{if result == expected, do: "Passed", else: "Failed"}")
      IO.puts("------")
    end)
  end
end

Main.main()
