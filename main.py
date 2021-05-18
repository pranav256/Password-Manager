from ui import Ui_MainWindow
from PyQt5.QtWidgets import QApplication, QMainWindow, QFileDialog, QMessageBox


class MainWindow(QMainWindow, Ui_MainWindow):
    def __init__(self, *args, **kwargs):
        super(MainWindow, self).__init__(*args, **kwargs)
        self.setupUi(self)
        self.show_all()
        self.add_button.clicked.connect(self.add_details)
        self.show()




    def add_details(self):
        u = self.user_name_entry.text()
        e = self.email_entry.text()
        p = self.password_entry.text()

        f = open("emails.txt", "a")

        f.write(u)
        f.write(",")
        f.write(e)
        f.write(",")
        f.write(p)
        f.write(",\n")

        self.listWidget_user_name.addItem(u)
        self.listWidget_email.addItem(e)
        self.listWidget_password.addItem(p)


    def show_all(self):
        f = open('emails.txt', 'r')
        for line in f:
            split = line.split(',')
            self.listWidget_user_name.addItem(split[0])
            self.listWidget_email.addItem(split[1])
            self.listWidget_password.addItem(split[2])
        f.close()



if __name__ == '__main__':
    app = QApplication([])
    window = MainWindow()
    app.exec_()
