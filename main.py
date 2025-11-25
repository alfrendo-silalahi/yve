import gi

gi.require_version("Gtk", "4.0")
from gi.repository import Gtk


class Window(Gtk.ApplicationWindow):
    def __init__(self, **kargs):
        super().__init__(**kargs, title="Hello world!")

        box  = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=10)
        self.set_child(box) 

        self.button = Gtk.Button(label="Click Here")
        self.button.connect('clicked', self.on_button_clicked)
        box.append(self.button)

        self.label = Gtk.Label(label="Label Component")
        box.append(self.label)
        
    def on_button_clicked(self, _widget):
        self.close()


def on_activate(app):
    # create window
    win = Window(application=app) 
    win.present()


# create new application
app = Gtk.Application(application_id="com.example.App")
app.connect("activate", on_activate)

# run application
app.run(None)
