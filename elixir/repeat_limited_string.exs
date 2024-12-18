defmodule Solution do
  @spec repeat_limited_string(s :: String.t(), repeat_limit :: integer()) :: String.t()
  def repeat_limited_string(s, repeat_limit) do
    heap =
      s
      |> String.graphemes()
      |> Enum.frequencies()
      |> Enum.map(fn {c, cnt} -> {c, cnt} end)
      |> Enum.sort_by(fn {char, _count} -> char end, :desc)

    build_string(heap, repeat_limit, "")
  end

  defp build_string([], _, result), do: result

  defp build_string([{char, count} | rest], limit, result) do
    repeat_count = min(count, limit)
    updated_result = result <> String.duplicate(char, repeat_count)
    remaining_count = count - repeat_count

    if remaining_count > 0 do
      case rest do
        [] ->
          updated_result

        [{next_char, next_count} | tail] ->
          if next_count > 0 do
            new_result = updated_result <> next_char
            next_remaining = next_count - 1

            updated_heap =
              []
              |> maybe_push({char, remaining_count})
              |> maybe_push({next_char, next_remaining})
              |> Kernel.++(tail)
              |> Enum.reject(fn {_c, co} -> co <= 0 end)
              |> Enum.sort_by(fn {c, _count} -> c end, :desc)

            build_string(updated_heap, limit, new_result)
          else
            updated_result
          end
      end
    else
      build_string(rest, limit, updated_result)
    end
  end

  defp maybe_push(heap, {_char, cnt} = item) when cnt > 0, do: [item | heap]
  defp maybe_push(heap, {_char, cnt}) when cnt <= 0, do: heap
end

defmodule Main do
  def main do
    test_cases = [
      %{
        s: "cczazcc",
        repeat_limit: 3,
        expected: "zzcccac"
      },
      %{
        s: "aababab",
        repeat_limit: 2,
        expected: "bbabaa"
      },
      %{
        s: "xyutfpopdynbadwtvmxiemmusevduloxwvpkjioizvanetecnuqbqqdtrwrkgt",
        repeat_limit: 1,
        expected: "zyxyxwxwvwvuvuvututstrtrtqpqpqponononmlmkmkjigifiededededcbaba"
      }
    ]

    test_cases
    |> Enum.each(fn %{s: s, repeat_limit: repeat_limit, expected: expected} ->
      result = Solution.repeat_limited_string(s, repeat_limit)

      IO.puts("Input: s = #{inspect(s)}, repeat_limit = #{inspect(repeat_limit)}")

      IO.puts("Output: #{inspect(result)}")
      IO.puts("Expected: #{inspect(expected)}")
      IO.puts("Test #{if result == expected, do: "Passed", else: "Failed"}")
      IO.puts("------")
    end)
  end
end

Main.main()
