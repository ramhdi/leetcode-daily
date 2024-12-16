defmodule Solution do
  @spec get_final_state(nums :: [integer], k :: integer, multiplier :: integer) :: [integer]
  def get_final_state(nums, k, multiplier) do
    1..k
    |> Enum.reduce(
      Enum.with_index(nums),
      fn _, acc ->
        {min_val, min_index} = Enum.min_by(acc, fn {val, _idx} -> val end)
        List.replace_at(acc, min_index, {min_val * multiplier, min_index})
      end
    )
    |> Enum.map(fn {val, _idx} -> val end)
  end
end

defmodule Main do
  def main do
    test_cases = [
      %{nums: [2, 1, 3, 5, 6], k: 5, multiplier: 2, expected: [8, 4, 6, 5, 6]},
      %{nums: [1, 2], k: 3, multiplier: 4, expected: [16, 8]}
    ]

    test_cases
    |> Enum.each(fn %{nums: nums, k: k, multiplier: multiplier, expected: expected} ->
      result = Solution.get_final_state(nums, k, multiplier)

      IO.puts(
        "Input: nums = #{inspect(nums)} k = #{inspect(k)} multiplier = #{inspect(multiplier)}"
      )

      IO.puts("Output: #{inspect(result)}")
      IO.puts("Expected: #{inspect(expected)}")
      IO.puts("Test #{if result == expected, do: "Passed", else: "Failed"}")
      IO.puts("------")
    end)
  end
end

Main.main()
