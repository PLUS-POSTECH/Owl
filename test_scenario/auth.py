import sys

if sys.argv[1].strip() == '1':
    print 'ok'
elif sys.argv[1].strip() == '2':
    print 'duplicate'
elif sys.argv[1].strip() == '3':
    print 'wrong'
elif sys.argv[1].strip() == '4':
    exit(1)
