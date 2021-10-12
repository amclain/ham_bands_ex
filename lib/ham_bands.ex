defmodule HamBands do
  use Rustler, otp_app: :ham_bands_ex, crate: :ham_bands

  defmodule Privilege do
    defstruct [:mode, :min, :max]
  end

  def list_bands(), do: :erlang.nif_error(:nif_not_loaded)
  def list_license_classes(), do: :erlang.nif_error(:nif_not_loaded)
  def list_frequency_privileges(_band, _license_class), do: :erlang.nif_error(:nif_not_loaded)
end
