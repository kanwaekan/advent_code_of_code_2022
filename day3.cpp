#include<bits/stdc++.h>

using namespace std;

void solve1(){
    string s;
    int score=0;

    while (cin>>s) {
        bool map[100]={0};

	for(int i=0;i<s.length()>>1;i++){
            map[s[i]-'A']=true;
	}

	for(int i=s.length()>>1;i<s.length();i++){
            if(map[s[i]-'A']){
                score+=s[i]-'Z'>0?s[i]-'a'+1:s[i]-'A'+27;
		break;
	    }
	}
    }

    cout<<score;
}


void solve2(){
    string s;
    int score=0;

    while (cin>>s) {
        bool map1[200]={false};
        bool map2[200]={false};
        bool map3[200]={false};

	for(int i=0;i<s.length();i++){
            map1[s[i]-'A']=true;
	}

	cin>>s;
	for(int i=0;i<s.length();i++){
            map2[s[i]-'A']=true;
	}

	cin>>s;
	for(int i=0;i<s.length();i++){
            map3[s[i]-'A']=true;
	}

	for(char i='a';i<='z';i++){
	    if(map1[i-'A']&& map2[i-'A']&& map3[i-'A']){
                score+=i-'a'+1;
		break;
	    }
        }

	for(char i='A';i<='Z';i++){
	    if(map1[i-'A']&& map2[i-'A']&& map3[i-'A']){
		cout<<(char)i;
                score+=i-'A'+27;
		break;
	    }
        }

    }

    cout<<score;
}
int main(){
	solve2();
}
