defmodule Solution do
  @spec find_champion(n :: integer, edges :: [[integer]]) :: integer
  def find_champion(n, edges) do
    # Initialize in-degrees to zero for all teams
    in_degrees =
      0..(n - 1)
      |> Enum.reduce(%{}, fn team, acc -> Map.put(acc, team, 0) end)

    IO.puts("in_degrees: #{inspect(in_degrees)}")

    # Update in-degrees based on the edges
    in_degrees =
      edges
      |> Enum.reduce(in_degrees, fn [_, v], acc -> Map.update!(acc, v, &(&1 + 1)) end)

    IO.puts("in_degrees: #{inspect(in_degrees)}")

    # Find the teams with in-degree of 0
    zeros =
      in_degrees
      |> Enum.filter(fn {_, in_degree} -> in_degree == 0 end)
      |> Enum.map(fn {team, _} -> team end)

    IO.puts("zeros: #{inspect(zeros)}")

    # Check if there is exactly one team with in-degree 0
    if length(zeros) == 1 do
      hd(zeros)
    else
      -1
    end
  end
end

defmodule Main do
  def main do
    test_cases = [
      %{n: 3, edges: [[0, 1], [1, 2]], expected: 0},
      %{n: 4, edges: [[0, 2], [1, 3], [1, 2]], expected: -1}
    ]

    Enum.each(test_cases, fn %{n: n, edges: edges, expected: expected} ->
      result = Solution.find_champion(n, edges)
      IO.puts("Input: n = #{n}, edges = #{inspect(edges)}")
      IO.puts("Output: #{result}")
      IO.puts("Expected: #{expected}")
      IO.puts("Test #{if result == expected, do: "Passed", else: "Failed"}")
      IO.puts("------")
    end)
  end
end

Main.main()
