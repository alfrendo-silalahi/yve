from itertools import count

import gi

gi.require_version("Gtk", "4.0")
from gi.repository import Gtk


class Header(Gtk.Box):
    def __init__(self, text):
        super().__init__(orientation=Gtk.Orientation.HORIZONTAL, spacing=10)

        # props
        self.text = text

        self.count = 1

        self.label = Gtk.Label(label=f'{text}-{self.count}')
        self.button = Gtk.Button(label="Click!")

        self.button.set_hexpand(True)
        self.button.set_halign(Gtk.Align.END)

        self.append(self.label)
        self.append(self.button)

        # set event
        self.button.connect('clicked', self.on_button_clicked)

    def on_button_clicked(self, e):
        self.count = self.count + 1

        # set new value for label state
        self.label.set_label(f'{self.text}-{self.count}')
