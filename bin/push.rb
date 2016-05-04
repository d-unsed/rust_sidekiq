require_relative '../lib/worker/printer'

10_000.times do
  Printer.perform_async('Stranger')
end
