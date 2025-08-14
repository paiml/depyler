# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpRYtlQW/f_WLkYks_60_.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
