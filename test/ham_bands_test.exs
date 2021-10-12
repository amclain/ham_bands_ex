defmodule HamBandsTest do
  use ExUnit.Case

  alias HamBands.Privilege

  test "can list bands" do
    assert HamBands.list_bands() |> Enum.member?("20m")
  end

  test "can_list_license_classes" do
    assert HamBands.list_license_classes() |> Enum.member?("E")
  end

  test "can_list_frequency_privileges" do
    expected_privilege = %Privilege{
      mode: "data",
      min: 7_000_000,
      max: 7_125_000
    }

    assert HamBands.list_frequency_privileges("40m", "E")
           |> Enum.member?(expected_privilege)
  end
end
