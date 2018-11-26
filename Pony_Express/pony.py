file = open("pony.in")

numberOfTests = int(file.readline())
citiesArray = []


def parseTest():
    global citiesArray

    vals = file.readline().split(' ')
    number_of_cities = int(vals[0])
    Q = int(vals[1])

    print(number_of_cities, Q)

    for city in range(number_of_cities):
        citiesArray.append([])
        vv = map(lambda x: x.strip(), file.readline().split(' '))
        citiesArray[city] = vv
        print citiesArray[city]

    print "----------------------------------------"
    for city in range(number_of_cities):
        vv = map(lambda x: x.strip(), file.readline().split(' '))
        citiesArray[city].insert(0, vv)
        print citiesArray[city]

    for _ in range(Q):
        file.readline()

for i in range(numberOfTests):
    print "Test nr " + str(i)
    citiesArray = []

    parseTest()

    if i > 0:
        break
