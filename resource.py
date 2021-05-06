def add(user_entry, email_entry, password_entry, list_user, list_email, list_password):
    u = user_entry.text()
    e = email_entry.text()
    p = password_entry.text()

    f = open("emails.txt", "a")

    f.write(u)
    f.write(",")
    f.write(e)
    f.write(",")
    f.write(p)
    f.write(",\n")

    list_user.addItem(u)
    list_email.addItem(e)
    list_password.addItem(p)


def show(listWidget_user, listWidget_email, listWidget_password):
    try:
        f = open('emails.txt', 'r')
    except FileNotFoundError:
        temp = open('emails.txt','w')
        temp.close()

    for line in f:
        split = line.split(',')
        listWidget_user.addItem(split[0])
        listWidget_email.addItem(split[1])
        listWidget_password.addItem(split[2])

    f.close()
