# https://leetcode.com/problems/reverse-odd-levels-of-binary-tree/
# 2024/12/20

Code.require_file("tree_node.ex", __DIR__)

defmodule Solution do
  @spec reverse_odd_levels(root :: TreeNode.t() | nil) :: TreeNode.t() | nil
  def reverse_odd_levels(nil), do: nil

  def reverse_odd_levels(root) do
    if root.left == nil or root.right == nil do
      root
    else
      {new_left, new_right} = dfs(root.left, root.right, 1)
      %TreeNode{root | left: new_left, right: new_right}
    end
  end

  defp dfs(nil, nil, _level), do: {nil, nil}
  defp dfs(nil, right, _level), do: {nil, right}
  defp dfs(left, nil, _level), do: {left, nil}

  defp dfs(left, right, level) do
    lval = left.val
    ll = left.left
    lr = left.right

    rval = right.val
    rl = right.left
    rr = right.right

    {new_left_val, new_right_val} =
      if rem(level, 2) == 1 do
        {rval, lval}
      else
        {lval, rval}
      end

    {updated_ll, updated_rr} = dfs(ll, rr, level + 1)
    {updated_lr, updated_rl} = dfs(lr, rl, level + 1)

    new_left = %TreeNode{val: new_left_val, left: updated_ll, right: updated_lr}
    new_right = %TreeNode{val: new_right_val, left: updated_rl, right: updated_rr}

    {new_left, new_right}
  end
end

defmodule Main do
  def main do
    test_cases = [
      %{
        root: [2, 3, 5, 8, 13, 21, 34],
        expected: [2, 5, 3, 8, 13, 21, 34]
      },
      %{
        root: [7, 13, 11],
        expected: [7, 11, 13]
      },
      %{
        root: [0, 1, 2, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2],
        expected: [0, 2, 1, 0, 0, 0, 0, 2, 2, 2, 2, 1, 1, 1, 1]
      }
    ]

    test_cases
    |> Enum.each(fn %{root: root, expected: expected} ->
      result =
        root
        |> TreeNodeHelpers.from_list()
        |> Solution.reverse_odd_levels()
        |> TreeNodeHelpers.to_list()

      IO.puts("Input: root = #{inspect(root)}")
      IO.puts("Output: #{inspect(result)}")
      IO.puts("Expected: #{inspect(expected)}")
      IO.puts("Test #{if result == expected, do: "Passed", else: "Failed"}")
      IO.puts("------")
    end)
  end
end

Main.main()
