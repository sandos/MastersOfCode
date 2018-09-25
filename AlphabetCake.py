file = open("alphabetlarge.in")

R = 0
C = 0
test_cases = 0

board = None

itr = 0
case = 1

def solve(m):
    #Pass 1 - Expand in both directions
    for l in m:
        lastC = " "
        for idx in range(len(l)):
            c = l[idx]
            if c == "?":
                if lastC != " ":
                    l[idx] = lastC
            else:
                lastC = c

        lastC = " "
        for idx in range(len(l)-1, -1, -1):
            # print idx
            c = l[idx]
            if c == "?":
                if lastC != " ":
                    l[idx] = lastC
            else:
                lastC = c

    #Pass 2 - fill in empty lines
    found = True
    while found:
        found = False
        for idx in range(len(m)-1):
            empty = True
            for c in m[idx]:
                if c != "?" and c != " ":
                    empty = False

            if empty:
                found = True
                #Copy from above or below
                #print "Empty line detected: " + str(l)
                #Copy from above
                m[idx] = m[idx+1]

        for idx in range(len(m)-1, 0, -1):
            empty = True
            for c in m[idx]:
                if c != "?" and c != " ":
                    empty = False

            if empty:
                found = True
                #Copy from above or below
                #print "Empty line detected: " + str(l)
                #Copy from above
                m[idx] = m[idx-1]


for line in file:
    spl = line.split(" ")
    if len(spl) == 1:
        if test_cases == 0 and len(spl[0]) > 0:
            test_cases = int(spl[0])
        elif R > 0 and C > 0:
            #print spl[0]
            board[itr] = list(spl[0].strip())
            # for u, c in enumerate(spl[0]):
            #     if c == "\n" or c == "\r" or c == " ":
            #         continue
            #     print "u: " + str(u) + " C: " + c + " itr: " + str(itr)
            #     map[itr][u] = c
            itr += 1
            if itr == R:
                # print "------------------ Done! --------------"
                # for l in map:
                    # print "".join(l)

                # print("-------------------------------------")
                solve(board)

                for l in board:
                    print "".join(l)

        else:
            print "Something wrong!"
    elif len(spl) == 2:
        # print "got two" + line
        R = int(spl[0])
        C = int(spl[1])
        itr = 0
        board = ["" for i in range(R)]

        print "Case #" + str(case) + ":"
        case += 1