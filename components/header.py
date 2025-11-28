import gi 

gi.require_version("Gtk", "4.0")
from gi.repository import Gtk

class Header(Gtk.Box):
    def __init__(self, text):
        super().__init__(orientation=Gtk.Orientation.HORIZONTAL, spacing=10)

        self.label = Gtk.Label(label=text)
        self.button = Gtk.Button(label="Klik!")

        self.button.set_hexpand(True)
        self.button.set_halign(Gtk.Align.END)

        self.append(self.label)
        self.append(self.button)
