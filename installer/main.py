import tkinter as tk
import os
import subprocess

def launch():
    try:
        subprocess.run(["git", "clone", "https://github.com/ThousandthStar/8bit-duels"], check=True)
        os.chdir(os.path.join("8bit-duels", "client"))
        os.system("cargo run")
    except Exception:
        label.configure(text="Downloading failed, defaulting to running")
        os.chdir(os.path.join("8bit-duels", "client"))
        os.system("cargo run")



def main():
    root = tk.Tk()
    label = tk.Label(text="8bit Duels Launcher")
    label.pack()
    launch_button = tk.Button(text="Launch", command=lambda: launch())
    launch_button.pack()
    root.title("8bit Duels Launcher")
    root.geometry("400x400")
    root.mainloop()

if __name__ == "__main__":
    main()
