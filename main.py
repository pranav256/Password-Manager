from ui import Ui_MainWindow
from PyQt5.QtWidgets import QApplication, QMainWindow


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

        encrypted_u = ""
        encrypted_e = ""
        encrypted_p = ""

        for letter in u:
            if letter == ' ':
                encrypted_u += ' '
            else:
                encrypted_u += chr(ord(letter) + 5)

        for letter in e:
            if letter == ' ':
                encrypted_e += ' '
            else:
                encrypted_e += chr(ord(letter) + 5)

        for letter in p:
            if letter == ' ':
                encrypted_p += ' '
            else:
                encrypted_p += chr(ord(letter) + 5)
        
        f = open("emails.txt", "a")

        f.write(encrypted_u + ',' + encrypted_e + ',' + encrypted_p + ', \n')

        self.listWidget_user_name.addItem(u)
        self.listWidget_email.addItem(e)
        self.listWidget_password.addItem(p)

    def show_all(self):
        f = open('emails.txt', 'r')
        for line in f:
            split = line.split(',')
            u = split[0]
            e = split[1]
            p = split[2]
            decrypted_u = ""
            decrypted_e = ""
            decrypted_p = ""
            for letter in u:
                if letter == ' ':
                    decrypted_u += ' '
                else:
                    decrypted_u += chr(ord(letter) - 5)

            for letter in e:
                if letter == ' ':
                    decrypted_e += ' '
                else:
                    decrypted_e += chr(ord(letter) - 5)

            for letter in p:
                if letter == ' ':
                    decrypted_p += ' '
                else:
                    decrypted_p += chr(ord(letter) - 5)

            self.listWidget_user_name.addItem(decrypted_u)
            self.listWidget_email.addItem(decrypted_e)
            self.listWidget_password.addItem(decrypted_p)

        f.close()


if __name__ == '__main__':
    app = QApplication([])
    window = MainWindow()
    app.exec_()
