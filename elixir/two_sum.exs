defmodule Solution do
  @spec two_sum(nums :: [integer], target :: integer) :: [integer]
  def two_sum(nums, target) do
    nums
    |> Enum.with_index()
    |> Enum.reduce_while(
      %{},
      fn {num, idx}, seen_map ->
        complement = target - num

        case Map.get(seen_map, complement) do
          nil -> {:cont, Map.put(seen_map, num, idx)}
          comp_idx -> {:halt, [comp_idx, idx]}
        end
      end
    )
  end
end

defmodule Main do
  def main do
    test_cases = [
      %{nums: [2, 7, 11, 15], target: 9, expected: [0, 1]},
      %{nums: [3, 2, 4], target: 6, expected: [1, 2]},
      %{nums: [3, 3], target: 6, expected: [0, 1]}
    ]

    Enum.each(test_cases, fn %{nums: nums, target: target, expected: expected} ->
      result = Solution.two_sum(nums, target)
      IO.puts("Input: nums = #{inspect(nums)}, target = #{target}")
      IO.puts("Output: #{inspect(result)}")
      IO.puts("Expected: #{inspect(expected)}")
      IO.puts("Test #{if result == expected, do: "Passed", else: "Failed"}")
      IO.puts("------")
    end)
  end
end

Main.main()
