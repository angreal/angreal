def pytest_itemcollected(item):
    if item._obj.__doc__:
        item._nodeid = item.obj.__doc__.strip()
