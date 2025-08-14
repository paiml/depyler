# GDB initialization script for Depyler debugging
# Source: /tmp/.tmp33aTyJ/YF4E_A_xoz8_ut_.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
