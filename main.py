import gi

gi.require_version("Gtk", "4.0")
from gi.repository import Gtk

from components.header import Header


class Window(Gtk.ApplicationWindow):
    def __init__(self, **kargs):
        super().__init__(**kargs, title="Hello world!")

        # set default window size
        self.set_default_size(1200, 800)

        box = Gtk.Box(orientation=Gtk.Orientation.VERTICAL, spacing=10) 

        header = Header(text="Yve HTTP Client")

        # add child components to parent component
        box.append(header) 
        
        self.set_child(box)
def on_activate(app):
    # create window
    win = Window(application=app) 
    win.present()


# create new application
app = Gtk.Application(application_id="com.example.App")
app.connect("activate", on_activate)

# run application
app.run(None)
