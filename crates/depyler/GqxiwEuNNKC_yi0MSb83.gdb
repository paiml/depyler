# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpTSG0ts/GqxiwEuNNKC_yi0MSb83.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
