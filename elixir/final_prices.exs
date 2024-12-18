# https://leetcode.com/problems/final-prices-with-a-special-discount-in-a-shop/
# 2024/12/18

# Iterative solution: 19 ms
# defmodule Solution do
#   @spec final_prices(prices :: [integer]) :: [integer]
#   def final_prices(prices) do
#     prices
#     |> Enum.with_index()
#     |> Enum.map(fn {curr_price, idx} ->
#       discount =
#         prices
#         |> Enum.slice(idx + 1, length(prices) - idx - 1)
#         |> Enum.find(0, fn x -> x <= curr_price end)

#       curr_price - discount
#     end)
#   end
# end

# Tail recursive solution: 1 ms
defmodule Solution do
  @spec final_prices(prices :: [integer]) :: [integer]
  def final_prices(prices) do
    calculate_final_prices(prices, [])
  end

  defp calculate_final_prices([], acc), do: Enum.reverse(acc)

  defp calculate_final_prices([current | rest], acc) do
    discount = Enum.find(rest, 0, fn x -> x <= current end)
    final_price = current - discount
    calculate_final_prices(rest, [final_price | acc])
  end
end

defmodule Main do
  def main do
    test_cases = [
      %{
        prices: [8, 4, 6, 2, 3],
        expected: [4, 2, 4, 2, 3]
      },
      %{
        prices: [1, 2, 3, 4, 5],
        expected: [1, 2, 3, 4, 5]
      },
      %{
        prices: [10, 1, 1, 6],
        expected: [9, 0, 1, 6]
      }
    ]

    test_cases
    |> Enum.each(fn %{prices: prices, expected: expected} ->
      result = Solution.final_prices(prices)
      IO.puts("Input: prices = #{inspect(prices)}")
      IO.puts("Output: #{inspect(result)}")
      IO.puts("Expected: #{inspect(expected)}")
      IO.puts("Test #{if result == expected, do: "Passed", else: "Failed"}")
      IO.puts("------")
    end)
  end
end

Main.main()
