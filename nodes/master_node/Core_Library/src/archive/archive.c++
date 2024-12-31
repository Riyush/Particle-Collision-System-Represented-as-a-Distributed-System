module;

#include <iostream>

export module Core_Test;

export class Core_Test {
	public:

		void PrintHelloWorld()
		{
			std::cout << "Hello World!  vkgyu\n";
			std::cin.get();
		}

};