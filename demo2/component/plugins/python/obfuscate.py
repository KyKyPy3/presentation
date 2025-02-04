import plugin
class Plugin(plugin.Plugin):
    def transform(self, input: str) -> str:
        result = ''.join(
            'x' if char in {'h', 'w', 'd'} else char for char in input
        )
        return result
