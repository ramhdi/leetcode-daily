defmodule Solution do
  @spec is_palindrome(x :: integer) :: boolean
  def is_palindrome(x) when x < 0, do: false
  def is_palindrome(0), do: true

  def is_palindrome(x) do
    rev = reverse(x, 0)
    rev == x
  end

  defp reverse(0, acc), do: acc
  defp reverse(x, acc), do: reverse(div(x, 10), acc * 10 + rem(x, 10))
end

defmodule Main do
  def main do
    test_cases = [
      %{x: 121, expected: true},
      %{x: -121, expected: false},
      %{x: 10, expected: false},
      %{x: 11, expected: true}
    ]

    Enum.each(test_cases, fn %{x: x, expected: expected} ->
      result = Solution.is_palindrome(x)
      IO.puts("Input: x = #{inspect(x)}")
      IO.puts("Output: #{inspect(result)}")
      IO.puts("Expected: #{inspect(expected)}")
      IO.puts("Test #{if result == expected, do: "Passed", else: "Failed"}")
      IO.puts("------")
    end)
  end
end

Main.main()
