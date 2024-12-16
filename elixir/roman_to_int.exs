defmodule Solution do
  numerals_and_values = [
    {"M", 1000},
    {"CM", 900},
    {"D", 500},
    {"CD", 400},
    {"C", 100},
    {"XC", 90},
    {"L", 50},
    {"XL", 40},
    {"X", 10},
    {"IX", 9},
    {"V", 5},
    {"IV", 4},
    {"I", 1}
  ]

  @spec roman_to_int(s :: String.t()) :: integer
  for {numeral, value} <- numerals_and_values do
    def roman_to_int(unquote(numeral) <> s), do: unquote(value) + roman_to_int(s)
  end

  def roman_to_int(""), do: 0
end

defmodule Main do
  def main do
    test_cases = [
      %{s: "III", expected: 3},
      %{s: "LVIII", expected: 58},
      %{s: "MCMXCIV", expected: 1994}
    ]

    Enum.each(test_cases, fn %{s: s, expected: expected} ->
      result = Solution.roman_to_int(s)
      IO.puts("Input: x = #{inspect(s)}")
      IO.puts("Output: #{inspect(result)}")
      IO.puts("Expected: #{inspect(expected)}")
      IO.puts("Test #{if result == expected, do: "Passed", else: "Failed"}")
      IO.puts("------")
    end)
  end
end

Main.main()
