# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpbtEldi/m4_Xok5F8_s20h_7_FhG.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
