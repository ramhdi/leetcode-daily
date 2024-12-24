# 771. Jewels and Stones
# https://leetcode.com/problems/jewels-and-stones/

# String: 3ms 71.4MB
defmodule Solution do
  @spec num_jewels_in_stones(jewels :: String.t(), stones :: String.t()) :: integer
  def num_jewels_in_stones(jewels, stones) do
    stones |> String.graphemes() |> Enum.count(fn stone -> String.contains?(jewels, stone) end)
  end
end

# Map: 7ms 71.2MB
# defmodule Solution do
#   @spec num_jewels_in_stones(jewels :: String.t(), stones :: String.t()) :: integer
#   def num_jewels_in_stones(jewels, stones) do
#     jewel_set = jewels |> String.graphemes() |> MapSet.new()
#     stones |> String.graphemes() |> Enum.count(fn stone -> MapSet.member?(jewel_set, stone) end)
#   end
# end

defmodule Main do
  def main do
    test_cases = [
      %{jewels: "aA", stones: "aAAbbbb", expected: 3},
      %{jewels: "z", stones: "ZZ", expected: 0}
    ]

    Enum.each(test_cases, fn %{jewels: jewels, stones: stones, expected: expected} ->
      result = Solution.num_jewels_in_stones(jewels, stones)
      IO.puts("Input: jewels = #{inspect(jewels)}, stones = #{inspect(stones)}")
      IO.puts("Output: #{result}")
      IO.puts("Expected: #{expected}")
      IO.puts("Test #{if result == expected, do: "Passed", else: "Failed"}")
      IO.puts("------")
    end)
  end
end

Main.main()
