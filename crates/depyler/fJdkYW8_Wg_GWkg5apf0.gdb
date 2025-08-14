# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpqarm9C/fJdkYW8_Wg_GWkg5apf0.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
