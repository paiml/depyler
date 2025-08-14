# GDB initialization script for Depyler debugging
# Source: /tmp/.tmp7jsrgQ/znt04J7q4a_57_knr4_.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
